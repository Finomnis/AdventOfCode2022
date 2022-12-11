pub mod nom;

#[cfg(test)]
macro_rules! day_input {
    ($day:ident) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            concat!("/input/2022/", concat!(stringify!($day), ".txt"))
        ))
    };
}

#[cfg(test)]
pub(crate) use day_input;
