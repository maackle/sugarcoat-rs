#[macro_export]
macro_rules! sugarcoat {
    (
        $coat:ident ( $wrapped:ident ) : $seed:ident
        { $( $field_name:ident : $field_type:ty => $field_constructor:expr),+ $(,)? }
    ) => {

        pub struct $coat($wrapped);

        impl $coat {
            $(
                pub fn $field_name(&self) -> &$field_type {
                    let f: Box<dyn Fn(&$wrapped) -> &$field_type> = Box::new($field_constructor);
                    f(&self.0)
                }
            )+
        }

        impl From<$wrapped> for $coat {
            fn from(w: $wrapped) -> Self {
                Self(w)
            }
        }

        impl From<$coat> for $wrapped {
            fn from(c: $coat) -> Self {
                c.0
            }
        }

        impl std::fmt::Debug for $coat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($coat))
                $(
                    .field(stringify!($field_name), self.$field_name())
                )+
                    .finish()
            }
        }

        impl PartialEq for $coat {
            fn eq(&self, other: &Self) -> bool {
                true
                $(
                    &&
                    self.$field_name() == other.$field_name()
                )+
            }
        }

        pub struct $seed {
            $(
                pub $field_name: $field_type,
            )+
        }

    };
}

#[cfg(test)]
mod tests {
    use arbitrary::Arbitrary;

    #[derive(Arbitrary)]
    #[allow(dead_code)]
    struct Data {
        x: u8,
        y: [u8; 4],
    }

    super::sugarcoat! {
        CoatedData(Data) : DataSeed {
            a: u8 => |foo| &foo.x,
            b: u8 => |foo| &foo.y[1],
        }
    }

    impl Data {
        pub fn from_seed(u: &mut arbitrary::Unstructured, seed: DataSeed) -> Self {
            let mut data = Data::arbitrary(u).unwrap();
            data.x = seed.a;
            data.y[1] = seed.b;
            data
        }
    }

    #[test]
    fn test_invariants() {
        let data1 = CoatedData::from(Data {
            x: 11,
            y: [11, 22, 33, 44],
        });
        let data2 = CoatedData::from(Data {
            x: 11,
            y: [100, 22, 23, 67],
        });

        // the two CoatedData structs are equivalent because the DataSeed only contains
        // `x` and the second element of the `y` array, which is the same in both
        assert_eq!(*data1.a(), 11);
        assert_eq!(*data2.a(), 11);
        assert_eq!(*data1.b(), 22);
        assert_eq!(*data2.b(), 22);
        assert_eq!(data1, data2);
        assert_eq!(
            format!("{:?}", data1),
            "CoatedData { a: 11, b: 22 }".to_string()
        );
        assert_eq!(
            format!("{:?}", data2),
            "CoatedData { a: 11, b: 22 }".to_string()
        );

        // We can construct an arbitrary `Data` from a `DataSeed`
        let mut u = arbitrary::Unstructured::new(&[0; 512]);
        let data3 = Data::from_seed(&mut u, DataSeed { a: 11, b: 22 }).into();
        assert_eq!(data1, data3);
        
        let data4 = Data::from_seed(&mut u, DataSeed { a: 11, b: 33 }).into();
        assert_ne!(data1, data4);
    }
}
