use std::{fs::File, path::Path};

pub(crate) fn _apply<F, A, B>(fun: F, args: A) -> B
where
    F: Fn(A) -> B,
{
    fun(args)
}

pub(crate) fn _compose<X, Y, Z, F, G>(f: F, g: G) -> impl Fn(X) -> Z
where
    F: Fn(X) -> Y,
    G: Fn(Y) -> Z,
{
    move |x| g(f(x))
}

pub(crate) fn _to_path(file_name: String) -> Result<File, std::io::Error> {
    File::create(Path::new(&file_name))
}
