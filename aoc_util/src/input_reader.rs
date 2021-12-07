use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::fmt;

pub fn read_line_into_vec<T>(path: &str, delimeter : char) -> Vec<T> 
where 
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut br = file_bufreader(path);
    let mut buf = String::new();
    br.read_line(&mut buf).expect("read line to vec failed");
    buf.split(delimeter).map(|x| x.parse::<T>().unwrap()).collect()
}

pub fn read_by_lines(path: &str) -> impl Iterator<Item = String>{
    let br = file_bufreader(path);
    br.lines().map(|l| l.unwrap()) 
}

fn file_bufreader(path: &str) -> BufReader<File> {
    BufReader::new(File::open(path).expect("cannot open file"))
}
