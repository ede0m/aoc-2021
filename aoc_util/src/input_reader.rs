use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::fmt;

pub fn read_line_into_vec<T>(path: &str, delimeter : Option<char>) -> Vec<T> 
where 
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut br = file_bufreader(path);
    let mut buf = String::new();
    br.read_line(&mut buf).expect("read line to vec failed");
    line_into_vec(buf, delimeter)
}

pub fn read_lines_into_vecs<T>(path: &str, delimeter : Option<char>) -> impl Iterator<Item = Vec<T>>
where 
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let lines = read_by_lines(path);
    lines.map(move |l| line_into_vec(l, delimeter))
}

pub fn read_by_lines(path: &str) -> impl Iterator<Item = String>{
    let br = file_bufreader(path);
    br.lines().map(|l| l.unwrap()) 
}

pub fn read_into_string(path: &str) -> String {
    let br = file_bufreader(path);
    let mut s = String::new();
    br.lines().for_each(|st| s.push_str(&st.unwrap()));
    s
}

fn file_bufreader(path: &str) -> BufReader<File> {
    BufReader::new(File::open(path).expect("cannot open file"))
}

fn line_into_vec<T>(buf: String, delimeter : Option<char>) -> Vec<T>
where 
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    if delimeter.is_some() {
        buf.split(delimeter.unwrap()).map(|x| x.parse::<T>().unwrap()).collect()
    }
    else {
        buf.chars().map(|x| x.to_string().parse::<T>().unwrap()).collect()
    }
}

