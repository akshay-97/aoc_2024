use std::collections::HashSet;
use std::hash::Hash;
use std::{ path::PathBuf, str::from_utf8};
use std::io::Read;

pub fn run(source : PathBuf) -> Result<(), anyhow::Error>{
    let mut f = std::fs::File::open(source)?;
    let mut vec : Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut vec);

    let str = from_utf8(&vec)?.to_owned();
    let mut grid = Grid::create(str);
    //println!("what is grid len {} {}", grid.grid.len(), grid.grid[0].len());
    grid.traverse(None);
    println!("what is part1 {}", grid.visited.len());
    let res = grid.place_block();
    println!("what is res {}", res);
    Ok(())
}

#[derive(Debug)]
struct Grid{
    grid : Vec<Vec<bool>>,
    visited : HashSet<(i32,i32,Dir)>,
    start_cord : (i32, i32),
    dir : Dir
}

#[derive(PartialEq,Eq, Hash,Clone, Copy,Debug)]
enum Dir{
    Up,Down, Left,Right
}

impl Grid{
    fn create(str : String) -> Self{
        let mut splitter = str.split("\n");
        let mut res = Vec::new();
        let mut start_cord : Option<(i32, i32)> = None;
        let mut row_index = 0;
        while let Some(s) = splitter.next(){
            let mut inner_vec = vec![false;s.len()];
            for (index, char) in s.chars().enumerate(){
                if char == '#'{
                    inner_vec[index] = true
                }
                if char == '^'{
                    start_cord = Some((row_index, index as i32));
                }
            }
            res.push(inner_vec);
            row_index +=1
        }
        Self{
            grid : res,
            start_cord : start_cord.expect("start not found"),
            dir : Dir::Up,
            visited : {
                let (r,c) = start_cord.unwrap();
                [(r,c, Dir::Up)].into_iter().collect()
            }
        }
    }

    fn traverse(&mut self, check_loop : Option<(usize,usize)>) -> bool{
        self.inner_traverse(self.start_cord.0 -1, self.start_cord.1, check_loop)
    }

    fn inner_traverse(&mut self, row : i32, col : i32, check_loop : Option<(usize,usize)>) -> bool{
        if row < 0 || row >= (self.grid.len() as i32) || col >= (self.grid[0].len() as i32) || col < 0{
            return false
        }
        if self.grid[row as usize][col as usize] || Some((row as usize,col as usize)) == check_loop{
            let (row, col) = self.rotate_clockwise(row,col);
            self.inner_traverse(row, col,check_loop)
        }
        else{
            if check_loop.is_some(){
                if !self.visited.insert((row,col, self.dir)){
                    return true
                }
            }else{
                self.visited.insert((row,col, Dir::Up));
            }
            let (row,col) = self.forward(row,col);
            self.inner_traverse(row, col,check_loop)
        }
    }

    fn rotate_clockwise(&mut self, row: i32, col: i32) -> (i32, i32){
        match self.dir{
            Dir::Down =>{
                self.dir = Dir::Left;
                self.forward(row-1, col)
            },
            Dir::Left => {
                self.dir = Dir::Up;
                self.forward(row,col+1)
            },
            Dir::Right =>{
                self.dir = Dir::Down;
                self.forward(row,col-1)
            },
            Dir::Up => {
                self.dir = Dir::Right;
                self.forward(row+1, col)
            },
        }
    }

    fn forward(&self, r: i32, c: i32) -> (i32,i32){
        match self.dir{
            Dir::Left => (r,c-1),
            Dir::Down => (r+1,c),
            Dir::Right => (r,c+1),
            Dir::Up => (r-1,c)
        }
    }

    fn place_block(&self) -> u32{
        let mut res = 0;
        for (r,c, _) in &self.visited{
            if (*r,*c) == self.start_cord{
                continue
            }
            let mut new_grid = Grid {
                grid : self.grid.clone(),
                start_cord : self.start_cord,
                dir : Dir::Up,
                visited : [(self.start_cord.0, self.start_cord.1, Dir::Up)].into_iter().collect()
            };
            //println!("what is grid {:?}", new_grid);
            if new_grid.traverse(Some((*r as usize, *c as usize))){
                res += 1;
            }
        }
        res
    }
}