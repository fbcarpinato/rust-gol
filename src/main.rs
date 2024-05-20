use rand::Rng;

#[derive(Clone, Copy)]
enum Cell {
    Live,
    Dead,
}

#[derive(Clone, Copy)]
struct CellPosition {
    x: usize,
    y: usize,
}

struct Conway {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    neighbors: Vec<Vec<Vec<CellPosition>>>,
}

impl Conway {
    fn new(width: usize, height: usize) -> Conway {
        let mut cells = vec![vec![Cell::Dead; height]; width];
        let mut rng = rand::thread_rng();

        for x in 0..width {
            for y in 0..height {
                if rng.gen::<f64>() > 0.5 {
                    cells[x][y] = Cell::Live;
                } else {
                    cells[x][y] = Cell::Dead;
                }
            }
        }

        let mut neighbors = vec![vec![vec![]; height]; width];
        for x in 0..width {
            for y in 0..height {
                let cell_neighbors = Conway::get_cell_neighbors(x, y, width, height);
                neighbors[x][y] = cell_neighbors;
            }
        }

        Conway {
            cells,
            width,
            height,
            neighbors,
        }
    }

    fn get_cell_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<CellPosition> {
        let mut neighbors = Vec::with_capacity(8);
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                neighbors.push(CellPosition {
                    x: nx as usize,
                    y: ny as usize,
                });
            }
        }

        neighbors
    }

    pub fn run_generation(&mut self) {
        let mut new_cells = vec![vec![Cell::Dead; self.height]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbors = self.neighbors[x][y].iter()
                    .filter(|n| matches!(self.cells[n.x][n.y], Cell::Live))
                    .count();

                new_cells[x][y] = match self.cells[x][y] {
                    Cell::Live if live_neighbors < 2 || live_neighbors > 3 => Cell::Dead,
                    Cell::Live => Cell::Live,
                    Cell::Dead if live_neighbors == 3 => Cell::Live,
                    Cell::Dead => Cell::Dead,
                };
            }
        }

        self.cells = new_cells;
    }

    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Live => print!("X"),
                    Cell::Dead => print!("-"),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut conway = Conway::new(20, 20);

    loop {
        print!("\x1B[2J\x1B[1;1H");
        conway.print();
        conway.run_generation();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
