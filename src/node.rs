use ndarray::s;
use ndarray::Array2;
use rand::Rng;

pub struct Node {
    //pub square: Vec<Vec<u16>>,
    pub square: Array2<u16>,
    //  chances: Vec<u16>,
    pub size: u16,
    pub total_chance: u16,
    pub total_chance_history: u32,
}

impl Node {
    pub fn new(size: u16) -> Node {
        let square = Array2::zeros((size as usize, size as usize)); // vec![vec![0; size as usize]; size as usize];
                                                                    //  let chances = vec![0; 2 * size as usize + 2];
        Node {
            square,
            //       chances,
            size,
            total_chance: 0,
            total_chance_history: 0,
        }
    }

    pub fn guess_chance(&mut self, calculate: bool) {
        // let prev_total_chance = self.total_chance;
        if calculate {
            let mut _sum: u16;
            let mut chance = 0;
            let magic_constant = (self.size * (self.size * self.size + 1)) / 2;
            // Calculate the sum of each row and column
            for i in 0..self.size {
                let row_sum: u16 = self.square.row(i as usize).sum();
                let col_sum: u16 = self.square.column(i as usize).sum();
                if row_sum == magic_constant {
                    chance += 1;
                }
                if col_sum == magic_constant {
                    chance += 1;
                }
            }

            // Calculate the sum of the diagonals
            let diag1_sum: u16 = self.square.diag().iter().sum();
            let diag2_sum: u16 = self.square.slice(s![..;-1, ..]).diag().iter().sum();
            if diag1_sum == magic_constant {
                chance += 1;
            }
            if diag2_sum == magic_constant {
                chance += 1;
            }

            self.total_chance = chance; // self.chances.iter().sum();
        }
    }

    pub fn is_result(&self) -> bool {
        self.total_chance == 2 * self.size + 2
    }

    pub fn get_chance_percent(&self) -> f32 {
        (self.total_chance as f32 / (2 * self.size + 2) as f32) * 100.0
    }

    pub fn swap_two_random_elements(&mut self, insist: InsistLevel) {
        let mut rng = rand::thread_rng();
        let i1: usize = rng.gen_range(0..self.size as usize);
        let j1: usize = rng.gen_range(0..self.size as usize);
        let i2: usize = rng.gen_range(0..self.size as usize);
        let j2: usize = rng.gen_range(0..self.size as usize);

        let prev_chance = self.total_chance;
        let temp = self.square[[i1, j1]];
        self.square[[i1, j1]] = self.square[[i2, j2]];
        self.square[[i2, j2]] = temp;
        self.guess_chance(true);
        match insist {
            InsistLevel::KeepEqual => {
                if prev_chance >= self.total_chance {
                    //Revert
                    self.square[[i2, j2]] = self.square[[i1, j1]];
                    self.square[[i1, j1]] = temp;
                    self.total_chance = prev_chance;
                    self.total_chance_history += 1;
                } else {
                    self.total_chance_history = 0;
                }
            }
            InsistLevel::ReplaceEqual => {
                if prev_chance > self.total_chance {
                    //Revert
                    self.square[[i2, j2]] = self.square[[i1, j1]];
                    self.square[[i1, j1]] = temp;
                    self.total_chance = prev_chance;
                    self.total_chance_history += 1;
                } else {
                    self.total_chance_history = 0;
                }
            }
            InsistLevel::ReplaceAnyway => {
                //Reset history, but allow more changes if history is too long
                if self.total_chance > 100 {
                    self.total_chance_history -= 100;
                } else {
                    self.total_chance_history = 0;
                }
            }
            InsistLevel::DontReplace => {
                //Revert
                self.square[[i2, j2]] = self.square[[i1, j1]];
                self.square[[i1, j1]] = temp;
                self.total_chance = prev_chance;
                self.total_chance_history = 0;
            }
        }
        //self.guess_chance(chance_changed);
        // self.total_chance_history.push_back(self.total_chance);
        // if buffer.total_chance > self.total_chance || insist == Any {
        //     self.square[first_element.0 as usize][first_element.1 as usize] = second_element_value;
        //     self.square[second_element.0 as usize][second_element.1 as usize] = first_element_value;
        //     self.total_chance = buffer.total_chance;
        // }
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.total_chance == other.total_chance
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        let mut square = Array2::zeros((self.size as usize, self.size as usize));
        for k in 0..self.size {
            for j in 0..self.size {
                square[[k as usize, j as usize]] = self.square[[k as usize, j as usize]];
            }
        }
        Node {
            square,
            //     chances: self.chances.clone(),
            size: self.size,
            total_chance: self.total_chance,
            total_chance_history: self.total_chance_history.clone(),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_chance.cmp(&other.total_chance)
    }
}

impl std::cmp::Eq for Node {}

#[allow(dead_code)]
pub enum InsistLevel {
    KeepEqual,
    ReplaceEqual,
    ReplaceAnyway,
    DontReplace,
}
