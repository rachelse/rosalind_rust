/* Construct the Suffix Tree of a String */

#[derive(PartialEq, Eq, Debug)]
pub struct Tree {
    first : Option<char>,
    rest : Vec<Option<Tree>>
}

impl Tree {
    fn new() -> Self {
        Tree {
            first : None,
            rest : Vec::new()
        }
    }

    fn find(&self, key: char) -> Option<usize> {
        for (i,tree) in self.rest.iter().enumerate() {
            if let Some(t) = tree {
                if t.first.unwrap() == key {
                    return Some(i)
                }
            }
        }
        return None
    }

    fn build_recursive(text:&str) -> Option<Self>{
        if text.len() < 1 {
            return None
        }
        let mut subtree= Tree::new();
        let first = text.chars().nth(0);
        let rest = &text[1..];
        subtree.first = first;
        let child = Tree::build_recursive(rest);
        subtree.rest.push(child);
        return Some(subtree)
    }

    fn construct(&mut self, text: &str) {
        if text.len() == 0 {
            return
        }
        let first = text.chars().nth(0).unwrap();
        if let Some(i) = self.find(first) {
            let mut subtree = self.rest.get_mut(i).unwrap().as_mut().unwrap();
            subtree.construct(&text[1..]);
        } else {
            let subtree = Tree::build_recursive(text);
            self.rest.push(subtree);
        }
    }
    
    fn is_tail(&self) -> bool {
        if self.rest.len() ==0 {
            return true
        } else if self.rest.len() == 1 && self.rest[0] == None {
            return true
        }
        false
    }

    fn is_root(&self) -> bool {
        if self.first == None {
            return true
        }
        false
    }

    fn print_child(&self) {
        for i in &self.rest {
            if let Some(tree) = i {
                tree.print();
            }
        }
    }

    fn print(&self) {
        if self.is_root() {
            self.print_child();
        } else if self.rest.len() > 1 {
            println!("{}", self.first.unwrap());
            self.print_child();
        } else if self.is_tail(){
            println!("{}", self.first.unwrap());
        } else {
            print!("{}", self.first.unwrap());
            self.print_child();
        }
    }
}

pub fn run(content: Vec<String>){
    let text = content.get(0).unwrap().to_owned();
    let mut tree = Tree::new();

    for i in 0..text.len() {
        tree.construct(&text[i..]);
    }
    tree.print();
}