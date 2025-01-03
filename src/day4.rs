use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}, ops::{Deref, DerefMut}, path::PathBuf};

pub fn run(source : PathBuf) -> Result<() , anyhow::Error>{
    let f = std::fs::File::open(source)?;
    let buffer = BufReader::new(f).lines();    
    println!("what is res {:?}" , process(buffer));
    Ok(())
}
/*
    x x x
    x x x
    x x x
    Vec<State>

    struct State{
        S A M X
        . M.
    }
*/

#[derive(Clone, Debug)]
struct InnerState{
    inner_state : [u32;9],
}

impl Deref for InnerState{
    type Target = [u32;9];
    fn deref(&self) -> &Self::Target {
        &self.inner_state
    }
}

impl DerefMut for InnerState{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_state
    }
}

enum Dir{
    Ver,
    Hor,
    LeftD,
    RightD,
}
impl InnerState{
    fn check_diag(&self) -> bool{
        if self[4] != 2{
            return false
        }
        match (self[0],self[8],self[2], self[6]){
            (1,3,1,3) => true ,
            (3,1,3,1) => true ,
            (3,1,1,3) => true ,
            (1,3,3,1) => true ,
            _ => false
        }
        
    }

    fn check(&self, lookup : usize , dir: Dir) -> (Option<usize>, Option<usize>){
        let mut res = (None, None);
        match dir{
            Dir::Ver => {
                if self[1] + 1 == (lookup as u32){
                    res.0 = Some(1)
                }
                if self[7] == (lookup as u32 + 1){
                    res.1 = Some(7)
                }
            },
            Dir::Hor =>{
                if self[3] + 1 == (lookup as u32){
                    res.0 = Some(3)
                }   
                if self[5] == (lookup as u32 + 1){
                    res.1 = Some(5)
                }
            },
            Dir::LeftD =>{ 
                if self[0] + 1 == (lookup as u32){
                    res.0 = Some(0)
                }
                if self[8] == (lookup as u32 + 1){
                    res.1 = Some(8)
                }
            },
            Dir::RightD =>{
                if self[2] + 1 == (lookup as u32){
                    res.0 = Some(2)
                }
                if self[6] == (lookup as u32 + 1){
                    res.1 = Some(6)
                }
            },
        }
        res
    }

    fn populate_for_x_s(&mut self, is_x: bool){
        if is_x{
            self.inner_state = [1;9]
        }else{
            self.inner_state = [3;9]
        }
    }
}

impl Default for InnerState{
    fn default() -> Self {
        Self{
            inner_state : [9;9]
        }
    }
}

struct State{
    prev : Vec<InnerState>,
    curr : Vec<InnerState>,
    mapper : HashMap<char, usize>,
}

impl State{
    fn init(size_hint: Option<usize>) -> Self{
        Self{
            prev : vec![InnerState::default(); size_hint.unwrap_or(0)],
            curr : vec![InnerState::default(); size_hint.unwrap_or(0)],
            mapper : HashMap::from([
                //('X', 1),
                ('M', 1),
                ('A', 2),
                ('S', 3),
            ])
        }
    }

    fn process(&mut self, index: usize, ch : char) -> u32{
        // // up
        // res += self.compare_and_update(Dir::Ver,ch, index);
        // // right up
        // res += self.compare_and_update(Dir::LeftD,ch, index);
        // // left up
        // res += self.compare_and_update(Dir::RightD,ch, index);
        // // left
        // res += self.compare_and_update(Dir::Hor,ch, index);
        let mut res = 0;
        match ch{
            'A' => {
                if index > 0 {
                    let inner = &self.prev[(index-1)];
                    if inner[4] == 1 || inner[4] ==3{
                        self.curr[index][0] = inner[4]
                    }
                }
                if let Some(inner) = self.prev.get(index+1){
                    if inner[4] == 1 || inner[4] ==3{
                        self.curr[index][2] = inner[4]
                    }
                }
                self.curr[index][4] = 2;
            },
            'M' | 'S' => {
                //check upper leftd
                if index > 0{
                    let inner = &self.prev[(index-1)];
                    if inner[4] == 2{
                        self.prev[index-1][8] = self.mapper[&ch] as u32;
                    }
                    if self.prev[index-1].check_diag(){
                        //println!("what is index {}", index -1);
                        res+= 1;
                    }
                }

                //check upper rightd
                if let Some(inner) = self.prev.get(index+1){
                    if inner[4] == 2{
                        self.prev[index+1][6] = self.mapper[&ch] as u32;
                    }
                }
            },
            _ => ()
        }

        self.update_init(ch,index);
        
        res
    }

    fn update_init(&mut self, ch: char, index : usize){
        match ch{
            'M' => self.curr[index].populate_for_x_s(true),
            'S' => self.curr[index].populate_for_x_s(false),
            _ => ()
        }
    }

    fn compare_and_update(&mut self, dir : Dir, ch : char, index: usize) -> u32{
        let val = self.mapper[&ch];
        let mut res = 0;
        let (up, down) = match dir{
            Dir::Hor if index > 0 => self.curr[index-1].check(val, dir),
            Dir::LeftD if index < (self.curr.len() - 1) => self.prev[index+1].check(val, dir),
            Dir::RightD if index > 0 => self.prev[index-1].check(val,dir),
            Dir::Ver => self.prev[index].check(val,dir),
            _ => (None, None)
        };

        if let Some(up) = up{
            self.curr[index][up] = val as u32;

            if ch == 'S'{
                res += 1
            } 
        }
        if let Some(down) = down{
            self.curr[index][down] = val as u32;
            if ch =='X'{
                res +=1
            }
        }
        res

    }

    fn reset_state(self) -> Self{
        let cap = self.curr.len();
        Self{
            prev : self.curr,
            curr : vec![InnerState::default(); cap],
            mapper : self.mapper
        }
    }

}

fn process(mut buffer : impl Iterator<Item = Result<String, std::io::Error>>) -> Option<u32>{
    let mut scan_str = buffer.next()?.ok()?;
    //println!("what is input {}", scan_str);
    let mut state = State::init(Some(scan_str.len()));
    let mut res = 0;
    
    loop{
        //println!("what is prev state: {:?}" ,state.prev);
        for (index, ch) in scan_str.char_indices(){
           // println!("ch {}" , ch == 'X');
            res += state.process(index, ch);
        }
        state = state.reset_state();
        if let Some(n_str) = buffer.next().and_then(|x| x.ok()){
            scan_str = n_str;
        }else{
            break;
        }
    } 
    Some(res)
}
/* 
struct State{
    acc :
}

*/

#[cfg(test)]
mod tests{
    use super::*;

    // #[test]
    // fn test1(){
    //     let vec : Vec<Result<String,std::io::Error>> = vec![Ok("XMAXMASAMXAS".to_string()), Ok("MAXSXAMAXAMA".to_string())];
    //     let  res = process(vec.into_iter());
    //     assert_eq!(None, res);
    // }

    #[test]
    fn test2(){
        //let vec : Vec<Result<String,std::io::Error>> = vec![Ok("XMAXMASAMXAS".to_string()), Ok("MAXSXAMAXAMA".to_string()), Ok("MAXSXAMAXAMA".to_string())];
        let nvec = vec![".M.S......","..A..MSMS.",".M.S.MAA..","..A.ASMSM.",".M.S.M....","..........","S.S.S.S.S.",".A.A.A.A..","M.M.M.M.M.",".........."];
        let vec : Vec<Result<String,std::io::Error>> = nvec.into_iter().map(|e| Ok(e.to_string())).collect();
        let res = process(vec.into_iter());
        assert_eq!(Some(9), res);
    }

    #[test]
    fn test3(){
        let nvec = vec!["MMMM", "AAAA", "SSSS", "AAAA", "MMMM", "AXXA", "SSSS", "AAAA", "MMMM"];
        let vec : Vec<Result<String,std::io::Error>> = nvec.into_iter().map(|e| Ok(e.to_string())).collect();
        let res = process(vec.into_iter());
        assert_eq!(Some(6), res);
    }

    #[test]
    fn test4(){
        let nvec = vec!["MMMMASSSS"];
        let vec : Vec<Result<String,std::io::Error>> = nvec.into_iter().map(|e| Ok(e.to_string())).collect();
        let res = process(vec.into_iter());
        assert_eq!(Some(0), res);
    }
}