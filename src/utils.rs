use std::{
    convert::AsRef,
    path::{Path, PathBuf},
};

pub fn as_path<P>(path: &P) -> &Path
where
    P: AsRef<Path> + ?Sized,
{
    path.as_ref()
}

// Inspired by `https://doc.rust-lang.org/stable/std/macro.matches.html`.
#[cfg(test)]
macro_rules! variant {
    ($expression_in:expr , $( $pattern:pat )|+ $( if $guard: expr )? $( => $expression_out:expr )? ) => {
        match $expression_in {
            $( $pattern )|+ $( if $guard )? => { $( $expression_out )? },
            variant => panic!("{:?}", variant),
        }
    };

    ($expression_in:expr , $( $pattern:pat )|+ $( if $guard: expr )? $( => $expression_out:expr)? , $panic:expr) => {
        match $expression_in {
            $( $pattern )|+ $( if $guard )? => { $( $expression_out )? },
            _ => panic!($panic),
        }
    };
}

macro_rules! dev_err {
    ($cause:expr) => {
        Error::DevError {
            line: line!(),
            file: file!(),
            cause: $cause.to_string(),
        }
    };
}

pub fn paint_file_name(mut path: PathBuf, color: ansi_term::Color) -> String {
    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
    let file_name = color.paint(file_name);

    path.pop();
    let rest = path.to_string_lossy();

    format!("{}/{}", rest, file_name)
}
