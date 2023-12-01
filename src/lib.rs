use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_input_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn read_input_to_vec(path: &str) -> Vec<String> {
    let input = read_input_file(path);
    let mut ret = vec![];
    for l in input.lines() {
        ret.push(l.to_string());
    }
    ret
}

pub fn read_input_to_vec_str<'a>(path: &'a str) -> Vec<String> {
    read_input_file(path)
        .lines()
        .map(|x| x.to_owned())
        .collect()
}

pub fn string_to_line_vec(input: String) -> Vec<String> {
    let mut ret = vec![];
    for l in input.lines() {
        ret.push(l.to_string());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
