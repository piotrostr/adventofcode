use std::collections::HashSet;
use std::fs::read_to_string;

type Point = (usize, usize);

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    lines.pop(); // pop the empty line at the end
    let arr = lines
        .iter()
        .map(|row| {
            row.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // create a set of cords to keep track of the visible trees
    let mut visible = HashSet::<Point>::new();

    // go in every direction to see if it is visible
    for i in 0..arr.len() {
        let mut largest = arr[i][0];
        visible.insert((i, 0));
        for j in 0..arr[i].len() {
            if arr[i][j] > largest {
                visible.insert((i, j));
                largest = arr[i][j];
            }
        }
    }

    // swap i and j to go vertical instead of horizontal
    for i in 0..arr.len() {
        let mut largest = arr[0][i];
        visible.insert((0, i));
        for j in 0..arr[i].len() {
            if arr[j][i] > largest {
                visible.insert((j, i));
                largest = arr[j][i];
            }
        }
    }

    // swap the order of left and right to go from the end
    // do the trick by subtracting the index from the length
    // be sure to subtract 1 from the length to account for the 0 index
    //
    // this can be refactored to use the rust `(0..arr.len()).rev()`,
    // did not know about it at time of writing lol
    for mut i in 0..arr.len() {
        i = arr.len() - 1 - i;
        let mut largest = arr[i][arr.len() - 1];
        visible.insert((i, arr.len() - 1));
        for mut j in 0..arr[i].len() {
            j = arr.len() - 1 - j;
            if arr[i][j] > largest {
                visible.insert((i, j));
                largest = arr[i][j];
            }
        }
    }

    for mut i in 0..arr.len() {
        i = arr.len() - 1 - i;
        let mut largest = arr[arr.len() - 1][i];
        visible.insert((arr.len() - 1, i));
        for mut j in 0..arr[i].len() {
            j = arr.len() - 1 - j;
            if arr[j][i] > largest {
                visible.insert((j, i));
                largest = arr[j][i];
            }
        }
    }

    println!("num visible: {}", visible.len());

    // part 2
    let mut top_score = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let next_score = get_scenic_score(&arr, (i, j));
            if next_score > top_score {
                top_score = next_score;
            }
        }
    }
    println!("top scenic score {}", top_score);
}

fn get_scenic_score(arr: &Vec<Vec<usize>>, point: Point) -> usize {
    let mut view_lengths = Vec::<usize>::new();
    let (x, y) = point;
    let tree = arr[x][y];

    // it crucial to increment the coord and view length at the same
    // time before comparison coord, as otherwise compare the tree to
    // initial tree at start => always break after first iter
    //
    // view length of blocked on first tree is 1
    // if it is going out of bounds it shall be 0
    // logical - it'd be a bad place for a treehouse
    {
        let mut x = x;
        let mut view_length = 0;
        while x != 0 {
            x -= 1;
            view_length += 1;
            if arr[x][y] >= tree {
                break;
            }
        }
        view_lengths.push(view_length);
    }

    {
        let mut x = x;
        let mut view_length = 0;
        while x < arr.len() - 1 {
            x += 1;
            view_length += 1;
            if arr[x][y] >= tree {
                break;
            }
        }
        view_lengths.push(view_length);
    }

    {
        let mut y = y;
        let mut view_length = 0;
        while y != 0 {
            y -= 1;
            view_length += 1;
            if arr[x][y] >= tree {
                break;
            }
        }
        view_lengths.push(view_length);
    }

    {
        let mut y = y;
        let mut view_length = 0;
        while y < arr.len() - 1 {
            y += 1;
            view_length += 1;
            if arr[x][y] >= tree {
                break;
            }
        }
        view_lengths.push(view_length);
    }

    return view_lengths
        .iter()
        .map(|x| *x)
        .reduce(|a, b| a * b)
        .unwrap();
}
