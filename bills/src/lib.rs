mod macros;

pub fn you_know() -> i32 {
    5
}

define_platform_enum! {
    WX => ("WX", "微信"),
    DY => ("DY", "抖音"),
    JB => ("JB", "借呗"),
    JD => ("JD", "京东白条"),
    JR => ("JR", "极融"),
    MT => ("MT", "美团"),
    JT => ("JT", "金条"),
    FQL => ("FQL", "分期乐"),
    ThreeSixty => ("360", "360"),
}
