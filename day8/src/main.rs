use utils::Content;

#[derive(Default, Debug)]
struct Grid {
    ht: usize,
    wd: usize,
    matrix: Vec<Vec<Tree>>,
}

impl Grid {
    fn put(&mut self, posx: usize, posy: usize, tree: Tree) {
        self.matrix[posx][posy] = tree;
    }
    fn put_row(&mut self, row: Vec<Tree>) {
        self.matrix.push(row);
    }
    fn set_dim(&mut self, ht: usize, wd: usize) {
        self.ht = ht;
        self.wd = wd;
    }
    fn len(&self) -> usize {
        self.matrix.len()
    }
    fn get(&self, posx: usize, posy: usize) -> Option<Tree> {
        if posx >= self.ht || posy >= self.wd {
            return None;
        }
        Some(self.matrix[posx][posy])
    }

    fn get_max_position_row(&self, row: usize, from: usize, to: usize) -> usize {
        let mut max_pos = from;
        let mut max_ht = self.matrix[row][from].ht;
        for y in from..to {
            let temp = self.matrix[row][y].ht;
            if temp >= max_ht {
                if temp == max_ht && from == 0 {
                    continue;
                } else {
                    max_ht = self.matrix[row][y].ht;
                    max_pos = y;
                }
            }
        }
        max_pos
    }

    fn get_max_position_col(&self, col: usize, from: usize, to: usize) -> usize {
        let mut max_pos = from;
        let mut max_ht = self.matrix[from][col].ht;
        for x in from..to {
            let temp = self.matrix[x][col].ht;
            if temp >= max_ht {
                if temp == max_ht && from == 0 {
                    continue;
                } else {
                    max_ht = self.matrix[x][col].ht;
                    max_pos = x;
                }
            }
        }
        max_pos
    }

    fn find_visibility_helper(&mut self, posx: usize, posy: usize) {
        let mut tree = self.get(posx, posy).unwrap();
        // check edge boundary
        if tree.posx == 0 || tree.posy == 0 || tree.posx == self.ht - 1 || tree.posy == self.wd - 1
        {
            tree.set_vis(true);
            self.put(posx, posy, tree);
            return;
        } else if self.get_max_position_row(tree.posx, 0, tree.posy + 1) == tree.posy {
            tree.set_vis(true);
            self.put(posx, posy, tree);
        } else if self.get_max_position_row(tree.posx, tree.posy, self.wd) == tree.posy {
            tree.set_vis(true);
            self.put(posx, posy, tree);
            return;
        } else if self.get_max_position_col(tree.posy, 0, tree.posx + 1) == tree.posx {
            tree.set_vis(true);
            self.put(posx, posy, tree);
            return;
        } else if self.get_max_position_col(tree.posy, tree.posx, self.ht) == tree.posx {
            tree.set_vis(true);
            self.put(posx, posy, tree);
            return;
        } else {
            return;
        }
    }
    fn find_visibility(&mut self) {
        for x in 0..self.ht {
            for y in 0..self.wd {
                self.find_visibility_helper(x, y);
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Tree {
    ht: u8,
    posx: usize,
    posy: usize,
    visible: bool,
}

impl Tree {
    fn new(ht: u8, posx: usize, posy: usize) -> Self {
        Tree {
            ht,
            posx,
            posy,
            visible: false,
        }
    }
    fn set_vis(&mut self, visibility: bool) {
        self.visible = visibility;
    }
}

fn main() {
    let data = Content::read_from_file("input.txt");
    let mut forest = Grid::default();
    let mut wd = 0;
    for (x, line) in data.enumerate() {
        let mut temp = Vec::<Tree>::new();
        for (y, ch) in line.chars().enumerate() {
            let ht = ch as u8 - '0' as u8;
            let tree = Tree::new(ht, x, y);
            temp.push(tree);
            wd += 1 & ((x == 0) as usize);
        }
        forest.put_row(temp);
    }
    forest.set_dim(forest.len(), wd);
    //dbg!(&forest);
    forest.find_visibility();
    let mut visible = 0usize;
    for row in &forest.matrix {
        for tree in row {
            visible += tree.visible as usize;
        }
    }
    dbg!(visible);
    //dbg!(&forest);
}
