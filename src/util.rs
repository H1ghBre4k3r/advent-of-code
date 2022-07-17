macro_rules! solution {
    ($($arg:tt)*) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = &name[..name.len() - 3];
        println!("\n{}: {}", name, $($arg)+);
    }};
}

pub(crate) use solution;
