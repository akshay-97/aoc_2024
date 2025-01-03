// file reader
use std::path::PathBuf;
use std::io::{self, BufRead};
#[derive(Debug)]
pub struct Content{
    col1 : Vec<u32>,
    col2 : Vec<u32>,
}

impl Content{
    pub fn populate(path: PathBuf) -> Result<Self, io::Error>{
        let f = std::fs::File::open(path.clone())?;
        let line_size = io::BufReader::new(f).lines().count();

        let f =  std::fs::File::open(path)?;
        let mut buffer = io::BufReader::new(f).lines();
        let mut col1 = Vec::with_capacity(line_size);
        let mut col2 = Vec::with_capacity(line_size);
        
        while let Some((n1,n2)) = consume_buffer(&mut buffer){
            col1.push(n1);
            col2.push(n2);
        }
        Ok(Self{
            col1, col2
        })
    }

    pub fn sort_col(&mut self){
        self.col1.sort();
        self.col2.sort();
    }

    pub fn calculate_distance(&self) -> u32{
        let len = self.col1.len();
        let mut res : u32 = 0;
        for i in 0..len{
            res += self.col1.get(i).unwrap().abs_diff(self.col2.get(i).unwrap().clone());
        }
        res
    }

    pub fn calculate_similarity(&self) -> u32{
        let mut l_index = 0;
        let mut r_index = 0;
        let cap = self.col1.len();
        
        let mut res = 0;

        while l_index < cap && r_index < cap{
            let mut exp = 0;
            while r_index < cap && self.col1[l_index] >= self.col2[r_index]{
                if self.col1[l_index] == self.col2[r_index]{
                    exp +=1;
                }
                r_index += 1;
            }
            res = res + self.col1[l_index]*exp;
            l_index += 1;
        }
        res
    }
}

use std::io::{Lines, BufReader};
use std::fs::File;

fn consume_buffer(buf : &mut Lines<BufReader<File>>) -> Option<(u32, u32)>{
    let line = buf.next()?.ok()?;
    let mut split = line.split("   ");
    let n1 : u32 = split.next()?.parse().ok()?;
    let n2 : u32 = split.next()?.parse().ok()?;
    Some((n1,n2))
}

pub fn run(path: PathBuf) -> Result<(), anyhow::Error>{
    let mut content = Content::populate(path)?;
    Content::sort_col(&mut content);
    let r = content.calculate_distance();
    println!("what is part1 {}", r);
    let r = content.calculate_similarity();
    println!("what is part2 {}", r);
    Ok(())
}