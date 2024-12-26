use std::collections::HashSet;
use std::{ path::PathBuf, str::from_utf8};
use std::io::Read;

pub fn run(source : PathBuf) -> Result<(), anyhow::Error>{
    let mut f = std::fs::File::open(source)?;
    let mut vec : Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut vec);

    let str = from_utf8(&vec)?.to_owned();
    let mut grid = Grid::create(str);
    println!("what is grid len {} {}", grid.grid.len(), grid.grid[0].len());
    grid.traverse();
    println!("what is res {}", grid.visited.len());
    Ok(())
}


struct Grid{
    grid : Vec<Vec<bool>>,
    visited : HashSet<(i32,i32)>,
    start_cord : (i32, i32),
    dir : Dir
}

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
            visited : [start_cord.unwrap()].into_iter().collect()
        }
    }

    fn traverse(&mut self){
        self.inner_traverse(self.start_cord.0 -1, self.start_cord.1)
    }

    fn inner_traverse(&mut self, row : i32, col : i32){
        if row < 0 || row >= (self.grid.len() as i32) || col >= (self.grid[0].len() as i32) || col < 0{
            return
        }
        if self.grid[row as usize][col as usize]{
            let (row, col) = self.rotate_clockwise(row,col);
            self.inner_traverse(row, col)
        }else{
            self.visited.insert((row,col));
            let (row,col) = self.forward(row,col);
            self.inner_traverse(row, col);
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
}