use std::fmt;
use std::io;
use std::string;

fn main() {
    let traffic_light = vec!((4,2), (4,3), (4,4), (4,5), (4,6));
    let galaxy = vec!(
        (3,3),(3,4),(3,5),(3,6),(3,7),(3,8),(3,10),(3,11),
        (4,3),(4,4),(4,5),(4,6),(4,7),(4,8),(4,10),(4,11),
        (5,10),(5,11),
        (6,3),(6,4),(6,10),(6,11),
        (7,3),(7,4),(7,10),(7,11),
        (8,3),(8,4),(8,10),(8,11),
        (9,3),(9,4),
        (10,3),(10,4),(10,6),(10,7),(10,8),(10,9),(10,10),(10,11),
        (11,3),(11,4),(11,6),(11,7),(11,8),(11,9),(11,10),(11,11)
    );

    let mut board = Board { width: 50, height: 20, live_cells: galaxy };

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
