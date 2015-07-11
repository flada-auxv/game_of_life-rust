use std::fmt;
use std::io;
use std::string;

fn main() {
    let live_cells = vec!((4,2), (4,3), (4,4), (4,5), (4,6));
    let mut board = Board { width: 10, height: 10, live_cells: live_cells };

    let mut reader = io::stdin();
    let string =  &mut String::new();

    board.display();

    loop {
        reader.read_line(string);

        if string.len() != 0 {
            board = board.tick();
            board.display();
        }
    }
}

struct Board {
    height: isize,
    width: isize,
    live_cells: Vec<(isize, isize)>
}

impl Board {
    fn all_cells(&self) -> Vec<(isize, isize)> {
        let mut all_cells = vec!();

        // map 使えなかった… orz
        for x in (0..self.height) {
            for y in (0..self.width) {
                all_cells.push((x, y));
            }
        }

        all_cells
    }

    fn tick(&self) -> Board {
        Board { height: self.height, width: self.width, live_cells: self.live_cells_next_go() }
    }

    fn display(&self) {
        println!("{:?}", self);
    }

    fn is_alive(&self, cell: (isize, isize)) -> bool {
        self.live_cells.iter().any(|live_cell| cell.eq(&live_cell))
    }

    fn live_cells_next_go(&self) -> Vec<(isize, isize)> {
        self.all_cells().iter().filter_map(|cell|
            match (self.is_alive(*cell), self.neighbours_count(*cell)) {
                (true, 2) => Some(*cell),
                (_, 3)    => Some(*cell),
                _ => None,
            }
        ).collect()
    }

    fn neighbours(&self, cell: (isize, isize)) -> Vec<(isize, isize)> {
        let (x, y) = cell;

        vec!(
            (x - 1, y - 1),
            (x - 1, y    ),
            (x - 1, y + 1),
            (x,     y - 1),
            (x,     y + 1),
            (x + 1, y - 1),
            (x + 1, y    ),
            (x + 1, y + 1)
        )
    }

    fn neighbours_count(&self, cell: (isize, isize)) -> isize {
        self.neighbours(cell).iter().filter(|&neighbour| self.is_alive(neighbour.clone())).count() as isize
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列結合えんどくさすぎぃぃぃぃぃぃぃぃっぃぃぃっぃ
        let mut board = vec!();
        let cells = self.all_cells();

        for x in (0..self.height) {
            for y in (0..self.width) {
                let idx = (x * self.width) + y;
                let cell_str = match self.is_alive(cells[idx as usize].clone()) {
                    true => "■",
                    false => "□"
                };
                board.push(cell_str);
            }
            board.push("\n")
        }

        write!(f, "{}", board.concat())
    }
}
