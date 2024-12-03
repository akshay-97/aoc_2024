use std::{io::{self, BufRead}, str::Split, path::PathBuf};

pub fn run(path : PathBuf) -> Result<(), anyhow::Error>{
    let f =  std::fs::File::open(path)?;
    let mut buffer = io::BufReader::new(f).lines();

    let mut res = 0;
    let mut res_old = 0;
    let mut diff= 0;

    while let Some(line) = buffer.next(){
        let line = line?;
        let split   = line.split(" ");
        let or  = safe(split);
        let split   = line.split(" ");
        let new = safe_(split);
        if new{
            res += 1;
        }

        if or != new{
            //println!("{}", line);
            diff += 1;
        }
        if or{
            res_old += 1;
        }
    }

    println!("day 2 part {} ", res);
    println!("day 2 diff 1 {} ", diff);
    Ok(())
}

fn safe_<'a>(splitter : Split<'a, &'a str>) -> bool{
    let arr : Vec<i32> = splitter.map(|el : &'a str| el.parse().expect("not number")).collect();
    let r = increasing(&arr) || decreasing(&arr);
    if !r{
        println!("{:?}", arr);
    }
    r
    

}

fn increasing<'a>(arr : &'a [i32]) -> bool{
    let mut prev = 0;
    let mut chance = true;
    for i in 1..arr.len(){
        //  a b c d - 
        //
        if arr[i] == arr[prev]{
            if !chance {return false}
            chance = false;
            continue;
        }else if arr[i] < arr[prev]{
            if !chance{return false}
            //skip prev
            if prev == 0{
                // check if skip i is needed
                if i+1 < arr.len() && arr[prev] < arr[i+1]{
                    chance = false;
                    continue;
                }
                prev = i;
                chance = false;
                continue;
            }
            if prev != 0{
                if arr[prev-1] < arr[i]{
                    chance = false;
                    //check diff
                    prev = i;
                    continue;
                }
            }
            
            // skip i
            chance = false;

        }else {
            let s = arr[i].abs_diff(arr[prev]);
            if s < 1 || s > 3{
                if !chance {return false}
                //skip prev
                if prev == 0{
                    if (i+1) < arr.len() && arr[i] < arr[i+1]{
                        chance = false;
                        prev= i;
                        continue;
                    }
                }
                //skip i
                chance = false;
                continue;
            }
            prev = i;
        }
    }
    return true
}
//37 30 32 30 28 26 23 22
fn decreasing<'a>(arr : &'a [i32]) -> bool{
    let mut prev = 0;
    let mut chance = true;
    for i in 1..arr.len(){
        //  a b c d - 
        //
        if arr[i] == arr[prev]{
            if !chance {return false}
            chance = false;
            continue;
        }else if arr[i] > arr[prev]{
            if !chance{return false}
            //skip prev
            if prev == 0{
                // check if skip is needed
                if i+1 < arr.len() && arr[prev] > arr[i+1]{
                    chance = false;
                    continue;
                }
                prev = i;
                chance = false;
                continue;
            }
            if prev != 0{
                if arr[prev-1] > arr[i]{
                    chance = false;
                    prev = i;
                    continue;
                }
            }
            
            // skip i
            chance = false;

        }else{
            let s = arr[i].abs_diff(arr[prev]);
            if s < 1 || s > 3{
                //skip prev
                //37 ,30 ,32 ,30 ,28 ,26 ,23 ,22
                if !chance {return false}
                if prev == 0{
                    if (i+1) < arr.len() && arr[i] > arr[i+1]{
                        chance = false;
                        prev= i;
                        continue;
                    }
                }
                //skip i
                chance = false;
                continue;
            }
            prev = i;
        }
    }
    return true
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn inc1(){
        let input = [1, 2, 3, 15, 25];
        assert_eq!(increasing(&input), false);
    }

    #[test]
    fn inc2(){
        let input = [2, 5, 1 ,2 ,3];
        assert_eq!(increasing(&input), false);
    }

    #[test]
    fn inc3(){
        let input = [0, 5, 1 ,2 ,3];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc4(){
        let input = [2, 5, 1 ,2 ,3];
        assert_eq!(increasing(&input), false);
    }

    #[test]
    fn inc5(){
        let input = [5,1,2,3];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc6(){
        let input = [1,2,5,4,6];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc7(){
        let input = [1,2,3,0];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc8(){
        let input = [1, 8 , 9, 10];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc9(){
        let input = [1, 6,3, 4, 5];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc10(){
        let input = [1, 6,3, 4, 5, 10];
        assert_eq!(increasing(&input), false);
    }

    #[test]
    fn inc11(){
        let input = [1, 2,3, 4, 5, 14];
        assert_eq!(increasing(&input), true);
    }

    #[test]
    fn inc12(){
        let input = [37 ,30 ,32 ,30 ,28 ,26 ,23 ,22];
        assert_eq!(increasing(&input), false);
    }

    #[test]
    fn dec12(){
        let input = [37 ,30 ,32 ,30 ,28 ,26 ,23 ,22];
        assert_eq!(decreasing(&input), false);
    }

    #[test]
    fn safe1(){
        let input = "4 2 7 9".split(" ");
        assert_eq!(safe_(input), true);
    }

}
fn safe<'a>(mut splitter : Split<'a ,&str>) -> bool{


    let mut f = cal_safe();
    f(&mut splitter).unwrap_or(false)
}

fn cal_safe() -> impl FnMut(&mut Split<'_, &str>) -> Option<bool>{

    |split| {
        let mut diff: i32 = 0;
        let mut is_tolerated = false;
        let mut prev : Option<i32> = None;
        /*
            in case of limit breach
                pp    p    c    cn
                pp c in limit

            7  3  4  10

         */
        while let Some(s) = split.next(){
            let n : i32 = s.parse().ok()?;
            if let Some(pre) = prev{
                if diff != 0 && (n - pre) * diff < 0{
                    return Some(false) 
                }
                if pre.abs_diff(n) < 1 || pre.abs_diff(n) > 3{
                    return Some(false)
                }
                diff = if pre > n { -1} else {1}; 

                prev = Some(n);
            }
            prev = Some(n);
        }
        Some(true)
    }
} 
