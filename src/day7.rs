use std::io::{BufReader, BufRead};
use std::path::PathBuf;
pub fn run(source : PathBuf) -> Result<() , anyhow::Error>{
    let f = std::fs::File::open(source)?;
    let buffer = BufReader::new(f).lines();    
    println!("what is res {:?}" , process(buffer));
    Ok(())
}
#[derive(Debug)]
struct Lne{
    target : u64,
    input  : Vec<u64>,
}

impl Lne{
    fn new(input_str : String) -> Option<Self>{
        let (target, rest) = input_str.split_once(':')?;
        let target = target.parse::<u64>().ok()?;
        let mut t = rest.split(' ');
        t.next();
        let input = t.map(|x| x.parse::<u64>().ok()).collect::<Option<Vec<u64>>>()?;
        Some(Self{target, input})
    }

    fn validate(&self) -> u64{
        if self.recurse(None, 0){
            return self.target
        }
        0
    }

    fn recurse(&self, curr : Option<u64>, index : usize) -> bool{
        //println!("{} {:?}" , self.target, curr);
        
        if curr.is_some() && curr.unwrap() == self.target && index == self.input.len(){ 
            return true
        }

        if index == self.input.len(){
            return false
        }

        if curr.is_some() && curr.unwrap() > self.target{ 
            return false
        }

        //try adding
        let add = self.recurse(Some(curr.unwrap_or(0) + self.input[index]), index + 1);
        //println!("what is add {}", add);
        if add {
            return true
        }

        //try multiplication
        let multi = self.recurse(Some(curr.unwrap_or(1) * self.input[index]), index + 1);

        if multi{
            return true
        }

        //try concat
        let concat = self.recurse(Some(concat_dig(curr.unwrap_or(0), self.input[index])), index+1);
        if concat{
            return true
        }

        return false
    }
}
fn process(mut buffer : impl Iterator<Item = Result<String, std::io::Error>>) -> u64{
    let mut res = 0;
    while let Some(str) = buffer.next().and_then(|x| x.ok()){
        if let Some(lne) = Lne::new(str){
            println!("{:?}", lne);
            res += lne.validate();
        }
    }
    res
}

fn concat_dig(mut a : u64, b : u64) -> u64{
    let mut copy_b = b;
    while copy_b > 0{
        a *= 10;
        copy_b /= 10;
    }
    a + b
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = "29: 10 19";
        let lne = Lne::new(input.to_string()).unwrap();
        assert_eq!(lne.recurse(None, 0), true);
    }

    #[test]
    fn test2(){
        let input = "190: 10 19";
        let lne = Lne::new(input.to_string()).unwrap();
        assert_eq!(lne.recurse(None, 0), true);
    }

    #[test]
    fn test3(){
        let input = "156: 1 5 6";
        let lne = Lne::new(input.to_string()).unwrap();
        assert_eq!(lne.recurse(None, 0), true);
    }

    #[test]
    fn test4(){
        let input = "7290: 6 8 6 15 6";
        let lne = Lne::new(input.to_string()).unwrap();
        assert_eq!(lne.recurse(None, 0), false);
    }

    #[test]
    fn test5(){
        let input = "156: 10 1";
        let lne = Lne::new(input.to_string()).unwrap();
        assert_eq!(lne.recurse(None, 0), false);
    }
}