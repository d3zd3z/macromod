// Macro escapement demo.

#![feature(macro_rules)]

pub mod stuff {
    #![macro_escape]

    #[derive(Show)]
    pub struct Thing {
        pub elt: u32,
    }

    impl Copy for Thing { }

    #[macro_export]
    macro_rules! thing {
        ($t:expr) => (Thing { elt: $t as u32 })
    }

    #[macro_export]
    macro_rules! see_blop {
        () => (blop())
    }

    pub fn test() -> Thing {
        thing!(4u8)
    }

    pub fn blop() -> &'static str {
        "inner blop"
    }
}
