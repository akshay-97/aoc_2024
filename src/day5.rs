use std::{array::IntoIter, cmp::Ordering, hash::Hash, io::{self, BufRead}, num::ParseIntError, path::PathBuf};
use std::collections::HashMap;
use std::{io::{BufReader, Lines}, fs::File};

pub fn run(path: PathBuf) -> Result<(), anyhow::Error>{
    let mut f = std::fs::File::open(path)?;
    let mut buffer = BufReader::new(f).lines();
    let state = State::init_mapper(&mut buffer)?;
    let res = state.process(buffer);
    println!("what is res {:?}", res);
    Ok(())
}

struct State{
    mapper : HashMap<u32, Vec<u32>>
}

impl State{
    fn init_mapper(buf : &mut impl Iterator<Item = Result<String, std::io::Error>>) -> Result<Self, anyhow::Error>{
        let mut mapper:HashMap<u32 , Vec<u32>> = HashMap::new();
        while let Some(dict) = buf.next(){
            let d = dict?;
            if d == ""{
                break
            }
            let mut split = d.split('|');
            let first_split_exists = split.next();
            if let Some(f) = first_split_exists{
                let first = f.parse::<u32>().ok()
                        .ok_or(anyhow::Error::msg("number parse error"))?;
                
                let second =
                    split
                        .next()
                        .and_then(|x| x.parse::<u32>().ok())
                        .ok_or(anyhow::Error::msg("number parse error"))?;
                if let Some(entry) = mapper.get_mut(&first){
                    entry.push(second);
                }else{
                    mapper.insert(first, vec![second]);
                }
            }else{
                break;
            }
        }
        Ok(Self { mapper })
    }
    
    fn process(&self, mut buf : Lines<BufReader<File>>) -> Option<u32>{
        let mut res = 0;
        while let Some(input) = buf.next(){
            let mut input_vec : Vec<u32> =
                input
                    .ok()
                    .and_then(|x| {
                        let split = x.split(',');
                        split
                            .map(|slice| slice.parse::<u32>())
                            .collect::<Result<Vec<u32>, ParseIntError>>()
                            .ok()
                    })?;
            let (result, _wrong_pos) = self.check_vec(&input_vec);
            if !result {
                self.sort_vec(&mut input_vec);
                let mid = (input_vec.len() +1) /2;
                res += input_vec[mid-1];
            }
        }

        
        Some(res)
    }


    fn sort_vec(&self, input_vec: &mut Vec<u32>){
        input_vec.sort_by(|a,b|{
            if self.compare(a,b){
                Ordering::Less
            }else{
                Ordering::Greater
            }
        });
    }

    fn check_vec(&self,input_vec: &Vec<u32>) -> (bool, usize){
        for i in 0..(input_vec.len() -1){
            if !self.compare(&input_vec[i], &input_vec[i+1]){
                return (false, i)
            }
        }
        return (true, 0)
    }

    fn compare(&self, first : &u32, second: &u32) -> bool{
        if let Some(v) = self.mapper.get(&first){
            if find(v, second){
                return true
            }
        }
        if let Some(v) = self.mapper.get(&second){
            if find(v, first){
                return false
            }
        }
        return true
    }
}


fn find(v : &Vec<u32>, elem : &u32) -> bool{
    for i in v{
        if *i ==  *elem{
            return true
        }
    }
    return false
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test] 
    fn test_sort(){
        let buf = vec!["47|53","97|13","97|61","97|47","75|29","61|13","75|53","29|13","97|29","53|29","61|53","97|53","61|29","47|13","75|47","97|75","47|61","75|61","47|29","75|13","53|13"];
        let vec : Vec<Result<String,std::io::Error>> = buf.into_iter().map(|e| Ok(e.to_string())).collect();
        let state = State::init_mapper(&mut vec.into_iter()).expect("state creation error");
        
        let mut input_vec = vec![75,97,47,61,53];
        state.sort_vec(&mut input_vec);

        assert_eq!(input_vec.as_slice(), [97,75,47,61, 53]);
    }
}