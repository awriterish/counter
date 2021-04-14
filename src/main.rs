use std::env;
use std::fs;
use std::process::exit;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn charAt(string: String, index: usize) -> char {
    let bytes = string.into_bytes();
    let c: char = bytes[index] as char;
    return c;
}

fn readFile(file: String) -> Vec<String> {
    let data = fs::read_to_string(file).expect("Unable to read file");
    let split = data.split("\n");
    let mut vec: Vec<String> = Vec::new();
    for s in split {
        vec.push(s.to_string());
    }
    return vec;
}

fn wordCount(files: Vec<String>) -> Vec<usize> {
    let mut totalWords:Vec<usize> =Vec::new();
    for file in files {
        let lines = readFile(file);
        let mut words = 0;
        for line in lines {
            let split = line.split(" ");
            for s in split {
                words+=1;
            }
        }
        totalWords.push(words);
    }
    return totalWords;
}

fn charCount(files: Vec<String>) -> Vec<usize> {
    let mut totalChars:Vec<usize> =Vec::new();
    for file in files {
        let lines = readFile(file);
        let mut chars = 0;
        for line in lines {
            chars += line.chars().count();
        }
        totalChars.push(chars);
    }
    return totalChars;
}

fn lineCount(files: Vec<String>) -> Vec<usize>{
    let mut totalLines:Vec<usize> =Vec::new();
    for file in files {
        totalLines.push(readFile(file).len());
    }
    return totalLines;
}

fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    if charAt(arg[0].clone(), 0) == '-' {
        let cmd = charAt(arg[0].clone(), 1);

        match cmd {
            'w' => {
                let totalWords = wordCount(arg[1..].to_owned());
                for i in 1..arg.len() {
                    println!("{} : {}",arg[i],totalWords[i-1]);
                }
            }
            'c' => {
                let totalChars = charCount(arg[1..].to_owned());
                for i in 1..arg.len() {
                    println!("{} : {}",arg[i],totalChars[i-1]);
                }
            }
            'l' => {
                let totalLines = lineCount(arg[1..].to_owned());
                for i in 1..arg.len() {
                    println!("{} : {}",arg[i],totalLines[i-1]);
                }
            }
            _ => {
                println!("Incompatible command");
            }
        }
    } else {
        let totalWords = wordCount(arg[0..].to_owned());
        let totalLines = lineCount(arg[0..].to_owned());
        let totalChars = charCount(arg[0..].to_owned());
        println!("File \t\t Lines \t Words \t Characters");
        for i in 0..arg.len() {
            println!("{} \t {} \t {} \t {}",arg[i],totalLines[i],totalWords[i],totalChars[i]);
        }
    }
}
