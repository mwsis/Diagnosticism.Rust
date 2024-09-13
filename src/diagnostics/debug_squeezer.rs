// src/diagnostics/debug_squeezer.rs : `DebugSqueezer`

use std::fmt as std_fmt;


/// Structure to assist with restricting the length of `Debug` forms of
/// fields within a given width.
pub struct DebugSqueezer<'a> {
    debugee : &'a dyn std_fmt::Debug,
    squeeze_width : usize,
}

// API functions
impl<'a> DebugSqueezer<'a> {
    pub fn new(
        debugee : &'a dyn std_fmt::Debug,
        squeeze_width : usize,
    ) -> Self {
        Self {
            debugee,
            squeeze_width,
        }
    }
}

// Mutating methods
impl<'a> DebugSqueezer<'_> {
}

// Nonmutating methods
impl<'a> DebugSqueezer<'_> {
}

// Trait implementations

impl<'a> std_fmt::Debug for DebugSqueezer<'_> {
    fn fmt(
        &self,
        f: &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {

        // NOTE: surely there's a better way to do this, perhaps using
        // `format_args!()`??

        let mut s = {

            if f.alternate() {
                if f.sign_plus() {
                    format!("{:+#?}", &self.debugee)
                } else {
                    format!("{:#?}", &self.debugee)
                }
            } else {
                if f.sign_plus() {
                    format!("{:+?}", &self.debugee)
                } else {
                    format!("{:?}", &self.debugee)
                }
            }
        };

        if s.len() > self.squeeze_width {

            let (width, trailing) = if s.len() <= self.squeeze_width {
                (self.squeeze_width, "")
            } else {
                if self.squeeze_width < 4 {
                    (0, "...")
                } else if self.squeeze_width < 6 {
                    (3, "...")
                } else {
                    if s.starts_with("{ ") {
                        (self.squeeze_width - 6, " ... }")
                    } else if s.starts_with("{") {
                        (self.squeeze_width - 5, " ...}")
                    } else {
                        (self.squeeze_width - 4, " ...")
                    }
                }
            };

            s.truncate(width);
            s.push_str(trailing);
        }

        f.write_str(&s)
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[derive(Debug)]
    struct ES {
        #[allow(unused)]
        x : u8,
    }

    mod TEST_STANDARD_Debug_SUPPORT_FOR_CustomType {
        #![allow(dead_code)]
        #![allow(non_snake_case)]

        use super::*;


            #[derive(Debug)]
            struct CustomType {
                i : i64,
                j : i64,
                s : String,
                e : ES,
            }


            #[test]
            fn TEST_Debug_FOR_CustomType() {

                // "vanilla"
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: 123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: 3 } }"#;
                    let actual = format!("{ct:?}");

                    assert_eq!(expected, actual);
                }

                // alternate
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: 123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: 3,
    },
}"#;
                    let actual = format!("{ct:#?}");

                    assert_eq!(expected, actual);
                }

                // sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: +123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: +3 } }"#;
                    let actual = format!("{ct:+?}");

                    assert_eq!(expected, actual);
                }

                // alternate | sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: +123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: +3,
    },
}"#;
                    let actual = format!("{ct:+#?}");

                    assert_eq!(expected, actual);
                }
            }
    }


    mod TEST_CUSTOM_Debug_SUPPORT_FOR_CustomType {
        #![allow(dead_code)]
        #![allow(non_snake_case)]

        use super::*;

            use std::fmt as std_fmt;


            struct CustomType {
                i : i64,
                j : i64,
                s : String,
                e : ES,
            }

            impl std_fmt::Debug for CustomType {
                fn fmt(&self, f: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
                    f.debug_struct("CustomType")
                        .field("i", &self.i)
                        .field("j", &self.j)
                        .field("s", &self.s)
                        .field("e", &self.e)
                        .finish()
                }
            }


            #[test]
            fn TEST_Debug_FOR_CustomType() {

                // "vanilla"
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: 123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: 3 } }"#;
                    let actual = format!("{ct:?}");

                    assert_eq!(expected, actual);
                }

                // alternate
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: 123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: 3,
    },
}"#;
                    let actual = format!("{ct:#?}");

                    assert_eq!(expected, actual);
                }

                // sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: +123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: +3 } }"#;
                    let actual = format!("{ct:+?}");

                    assert_eq!(expected, actual);
                }

                // alternate | sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: +123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: +3,
    },
}"#;
                    let actual = format!("{ct:+#?}");

                    assert_eq!(expected, actual);
                }
            }
    }


    mod TEST_CUSTOM_SQUEEZED_Debug_SUPPORT_FOR_CustomType_WITH_OVERLONG_WIDTH {
        #![allow(dead_code)]
        #![allow(non_snake_case)]

        use super::*;

            use super::super::DebugSqueezer;

            use std::fmt as std_fmt;

            const SQUEEZE_WIDTH : usize = 100;


            struct CustomType {
                i : i64,
                j : i64,
                s : String,
                e : ES,
            }

            impl std_fmt::Debug for CustomType {
                fn fmt(&self, f: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
                    f.debug_struct("CustomType")
                        .field("i", &DebugSqueezer::new(&self.i, SQUEEZE_WIDTH))
                        .field("j", &DebugSqueezer::new(&self.j, SQUEEZE_WIDTH))
                        .field("s", &DebugSqueezer::new(&self.s, SQUEEZE_WIDTH))
                        .field("e", &DebugSqueezer::new(&self.e, SQUEEZE_WIDTH))
                        .finish()
                }
            }


            #[test]
            fn TEST_Debug_FOR_CustomType() {

                // "vanilla"
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: 123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: 3 } }"#;
                    let actual = format!("{ct:?}");

                    assert_eq!(expected, actual);
                }

                // alternate
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: 123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: 3,
    },
}"#;
                    let actual = format!("{ct:#?}");

                    assert_eq!(expected, actual);
                }

                // sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: +123456789, j: -9999999, s: "abcdefghijklmnopqrstuvwxyz", e: ES { x: +3 } }"#;
                    let actual = format!("{ct:+?}");

                    assert_eq!(expected, actual);
                }

                // alternate | sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: +123456789,
    j: -9999999,
    s: "abcdefghijklmnopqrstuvwxyz",
    e: ES {
        x: +3,
    },
}"#;
                    let actual = format!("{ct:+#?}");

                    assert_eq!(expected, actual);
                }
            }
    }


    mod TEST_CUSTOM_SQUEEZED_Debug_SUPPORT_FOR_CustomType_WITH_TRUNCATING_WIDTH {
        #![allow(dead_code)]
        #![allow(non_snake_case)]

        use super::*;

            use super::super::DebugSqueezer;

            use std::fmt as std_fmt;

            const SQUEEZE_WIDTH : usize = 8;


            struct CustomType {
                i : i64,
                j : i64,
                s : String,
                e : ES,
            }

            impl std_fmt::Debug for CustomType {
                fn fmt(&self, f: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
                    f.debug_struct("CustomType")
                        .field("i", &DebugSqueezer::new(&self.i, SQUEEZE_WIDTH))
                        .field("j", &DebugSqueezer::new(&self.j, SQUEEZE_WIDTH))
                        .field("s", &DebugSqueezer::new(&self.s, SQUEEZE_WIDTH))
                        .field("e", &DebugSqueezer::new(&self.e, SQUEEZE_WIDTH))
                        .finish()
                }
            }


            #[test]
            fn TEST_Debug_FOR_CustomType() {

                // "vanilla"
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: 1234 ..., j: -9999999, s: "abc ..., e: ES { ... }"#;
                    let actual = format!("{ct:?}");

                    assert_eq!(expected, actual);
                }

                // alternate
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: 1234 ...,
    j: -9999999,
    s: "abc ...,
    e: ES { ...,
}"#;
                    let actual = format!("{ct:#?}");

                    assert_eq!(expected, actual);
                }

                // sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType { i: +123 ..., j: -9999999, s: "abc ..., e: ES { ... }"#;
                    let actual = format!("{ct:+?}");

                    assert_eq!(expected, actual);
                }

                // alternate | sign_plus
                {
                    let ct = CustomType { i: 123456789, j: -9999999, s : "abcdefghijklmnopqrstuvwxyz".into(), e : ES { x: 3 } };

                    let expected = r#"CustomType {
    i: +123 ...,
    j: -9999999,
    s: "abc ...,
    e: ES { ...,
}"#;
                    let actual = format!("{ct:+#?}");

                    assert_eq!(expected, actual);
                }
            }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
