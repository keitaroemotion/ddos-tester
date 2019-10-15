use std::env;

pub fn parse() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let options           = args
                                .into_iter()
                                .filter(|arg| arg.starts_with("--"))
                                .collect();

    let args: Vec<String> = env::args().collect();
    let _args             = args
                                .into_iter()
                                .filter(|arg| !arg.starts_with("--"))
                                .collect();

    return (options, _args)
}
