use std::{io::{self, BufRead, BufReader, Read}, path::PathBuf, str::Split};
use std::fs::File;
use std::str::from_utf8;
pub fn run(path : PathBuf) -> Result<(), anyhow::Error>{
    let mut f =  std::fs::File::open(path)?;
    let mut vec: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut vec);

    let str = from_utf8(&vec)?.to_owned();
    let splitter = str.split("mul(");

    let mut fres = 0;
    let mut splitter = Splitter::new(splitter);
    while let Some(res) = splitter.next(){
        if let Ok(r) = res{
            fres += r;
        }
    }
   
    println!("what is res {:?}" ,fres);
    Ok(())
}

struct Splitter<'a>{
    split : Split<'a, &'a str>,
    enabled : bool,
}

impl<'a> Splitter<'a>{
    fn check_disabled(&mut self, input : &str){
        if input.contains("don't()"){
            self.enabled = false;
        }
    }

    fn check_enabled(&mut self, input : &str){
        if input.contains("do()"){
            self.enabled = true;
        }
    }

    fn new(split : Split<'a, &'a str>) -> Self{
        Self{
            split,
            enabled : true
        }
    }
}
/*
    mul(...
    don'tmul(..
    ..mul(..
    
*/
impl <'a> Iterator for Splitter<'a>{
    type Item = Result<i64,()>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.enabled{
            let a = self.split.next()?;
            let mut pointer = &a; 
            let opt_res = get_mult_res(&mut pointer);
            self.check_disabled(a);
            if let Some((f,s)) = opt_res{
                return Some(Ok(f*s))
            }
            Some(Err(()))
        }else{
            let a = self.split.next()?;
            self.check_enabled(a);
            Some(Err(()))
        }
    }
}
fn get_mult_res<'a>(until_next: &'a str) -> Option<(i64, i64)>{
    println!("what is res until_next {}", until_next);
    let (first, second) = until_next.split_once(",")?;
    let f = first.parse::<i64>().ok()?;
    let (num, _) : (&'a str, &'a str)= second.split_once(")")?;
    let s = num.parse::<i64>().ok()?;
    Some((f,s))
}