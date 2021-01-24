use std::fs;

pub fn sample_inputs() -> Vec<String>
{
    let ret_inputs = Vec::new();

    let paths = fs::read_dir("../sample_inputs").unwrap();

    for path in paths {
        println!("{:?}", path);
    }

    ret_inputs
}

