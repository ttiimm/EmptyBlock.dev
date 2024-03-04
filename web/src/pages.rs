macro_rules! page {
    ($page:ident) => {
        pub mod $page;
        pub use $page::*;
    };
}

page!(home);
page!(tiles);