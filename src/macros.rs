#[macro_export]
macro_rules! static_files {
    ($(($name:ident, $file_path:expr, $mime:expr)),* $(,)?) => {
        use mime::Mime;

        #[derive(Debug)]
        pub struct StaticFile {
            pub content: &'static [u8],
            pub name: &'static str,
            pub mime: &'static Mime,
        }

        impl StaticFile {
            /// Get a single `StaticFile` by name, if it exists.
            #[must_use]
            pub fn get(name: &str) -> Option<&'static Self> {
                if let Ok(pos) = STATICS.binary_search_by_key(&name, |s| s.name) {
                    Some(STATICS[pos])
                } else {
                    None
                }
            }
        }

        lazy_static::lazy_static! {
            $(
                pub static ref $name: StaticFile = {
                    let file_name = $file_path.rsplit('/').next().unwrap();
                    let (base_name, extension) = file_name.rsplit_once('.').unwrap();

                    StaticFile {
                        content: include_bytes!($file_path),
                        name: Box::leak(format!("{}-{}.{}",
                                base_name,
                                env!(concat!(stringify!($name), "_HASH")),
                                extension
                            ).into_boxed_str()),
                        mime: &$mime,
                    }
                };
            )*

            pub static ref STATICS: Vec<&'static StaticFile> = {
                let mut v = vec![
                    $(
                        &*$name,
                    )*
                ];
                v.sort_by_key(|s| s.name);
                v
            };
        }
    };
}
