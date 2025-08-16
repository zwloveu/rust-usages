#[macro_export]
macro_rules! define_platform_enum {
    (
        $(
            $variant: ident => ($code: expr, $chinese_name: expr),
        )*
    ) => {
        #[derive(Debug, Clone)]
        pub struct PlatformInfo {
            code: String,
            chinese_name: String,
        }

        #[derive(Debug, Clone)]
        pub enum Platform {
            $(
                $variant(PlatformInfo),
            )*
        }

        impl Platform {
            pub fn code(&self) -> &str {
                match self {
                    $(
                        Self::$variant(info) => &info.code,
                    )*
                }
            }

            pub fn chinese_name(&self) -> &str {
                match self {
                    $(
                        Self::$variant(info) => &info.chinese_name,
                    )*
                }
            }

            pub fn from_code(code: &str) -> Result<Self, String> {
                let code_upper = code.to_uppercase();
                
                match code_upper.as_str() {
                    $(
                        val if val == $code.to_uppercase().as_str() => Ok(Self::$variant(PlatformInfo {
                            code: val.to_string(),
                            chinese_name: $chinese_name.to_string(),

                        })),
                    )*
                    _ => Err(format!("unknown platform code: {}", code)),
                }
            }
        }
    };
}