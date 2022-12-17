use std::collections::HashMap;
use std::fs::read_to_string;

/// Main component of the solution is to increment all of the directory sizes with every file in
/// the directory
///
/// e.g. for /a/b/c:
///
/// if file f in c: a += f, b += f
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut path = "/".to_string();
    let mut directories = HashMap::<String, i32>::new();

    directories.insert(path.clone(), 0);

    for line in input.split("\n") {
        let first_element = *line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .nth(0)
            .unwrap();
        if first_element == "$" {
            let second_element = *line
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .nth(1)
                .unwrap();
            if second_element == "cd" {
                let dir_name = line.split(" ").nth(2).unwrap();
                if dir_name == "/" {
                    continue;
                }
                if dir_name == ".." {
                    path = strip_dir(path);
                } else {
                    path = append_dir(path, dir_name.to_string());
                }
            }
        } else {
            // check if first element is file-size
            match first_element.parse::<i32>() {
                Ok(file_size) => increment_recursively(&mut directories, path.clone(), file_size),
                Err(_) => {}
            }
        }
    }
    let solution: i32 = directories
        .values()
        .filter(|v| **v <= 100000)
        .map(|x| *x)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{}", solution)
}

// increment_recursively increments all of the parent directories in the map until / is reached
fn increment_recursively(directories: &mut HashMap<String, i32>, path: String, file_size: i32) {
    let mut path = path.clone();
    // increment root /, always
    *directories.entry("/".to_string()).or_insert(0) += file_size;
    while path != "/" {
        if path == "" {
            break;
        }
        // increment the last path in the mapping
        *directories.entry(path.clone()).or_insert(0) += file_size;
        path = strip_dir(path);
    }
}

fn strip_dir(path: String) -> String {
    let mut path = path.clone();
    if path == "/" {
        return path;
    }
    let mut tmp = path.split("/").collect::<Vec<&str>>();
    tmp.pop();
    path = tmp.join("/");
    if path == "" {
        return "/".to_string();
    }
    return path;
}

fn append_dir(path: String, dir_name: String) -> String {
    if path == "/" {
        return format!("{}{}", path, dir_name);
    }
    let mut path = path.clone();
    let mut tmp = path.split("/").collect::<Vec<&str>>();
    tmp.push(dir_name.as_str());
    path = tmp.join("/");
    return path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_dir_works() {
        let path = "/a/b/c".to_string();
        let res = strip_dir(path);
        assert_eq!(res, "/a/b".to_string());
    }

    #[test]
    fn strip_dir_works_with_root() {
        let path = "/".to_string();
        let res = strip_dir(path);
        assert_eq!(res, "/".to_string());
    }

    #[test]
    fn strip_dir_works_with_short_path() {
        let path = "/a".to_string();
        let res = strip_dir(path);
        assert_eq!(res, "/".to_string());
    }

    #[test]
    fn append_dir_works() {
        let path = "/a/b/c".to_string();
        let res = append_dir(path, "d".to_string());
        assert_eq!(res, "/a/b/c/d".to_string());
    }

    #[test]
    fn append_dir_works_with_root() {
        let path = "/".to_string();
        let res = append_dir(path, "a".to_string());
        assert_eq!(res, "/a".to_string());
    }
}
