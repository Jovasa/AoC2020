use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, EnumIter)]
enum Edge {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

type TileRef = Rc<RefCell<Tile>>;

struct Tile {
    id: u32,
    data: Vec<String>,
    neighbors: HashMap<Edge, Option<TileRef>>,
    orientation: u32,
    left_edge: String,
    right_edge: String,
}

impl Tile {
    fn new(s: &str) -> TileRef {
        let mut all_data = s.split("\n").into_iter();
        let id = all_data.next().unwrap()[5..10].parse::<u32>().unwrap();

        let mut data: Vec<String> = Vec::new();
        for line in all_data {
            data.push(line.to_owned());
        }
        let left_edge = data.iter().map(|x| x.chars().nth(0).unwrap()).collect::<String>();
        let right_edge = data.iter().map(|x| x.chars().last().unwrap()).collect::<String>();

        let mut neighbors: HashMap<Edge, Option<TileRef>> = HashMap::new();
        for edge in Edge::iter() {
            neighbors.insert(edge, None);
        }

        Rc::new(RefCell::new(Tile {
            id, data,
            neighbors,
            orientation: 0,
            left_edge,
            right_edge
        }))
    }
    fn get_edge(self, edge: &Edge) -> String{
        let t = match edge {
            Edge::TOP => self.data.first().unwrap().to_owned(),
            Edge::BOTTOM => self.data.last().unwrap().to_owned(),
            Edge::LEFT => self.left_edge,
            Edge::RIGHT => self.right_edge
        };
        t
    }
}

fn match_edges(mut first: TileRef, mut other: TileRef) -> bool {
    for my_edge in Edge::iter()  {
        match first.borrow().neighbors.get(&my_edge).unwrap() {
            None => continue,
            _ => ()
        }
        for other_edge in Edge::iter()  {
            match other.borrow().neighbors.get(&other_edge).unwrap() {
                None => continue,
                _ => ()
            }
            if first.borrow().get_edge(&my_edge) == other.borrow().get_edge(&other_edge) {
                first.borrow_mut().neighbors.insert(my_edge, Some(other));
                other.borrow_mut().neighbors.insert(other_edge, Some(first));
                return true;
            }
        }
    }
    false
}

fn main() {

}