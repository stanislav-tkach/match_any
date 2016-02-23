#[macro_export]
macro_rules! match_any {
    ($a:ident => $($x:ident: $t:ty => $b:block),+) => {
        {
            $(
                if let Some($x) = $a.downcast_ref::<$t>() {
                    $b
                };
            )+
        }
    }
}
