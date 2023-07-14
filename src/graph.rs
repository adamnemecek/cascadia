// //
// // graph!(1 -> 2, 2 -> 3)
// //
// macro_rules! graph {
//     () => {
//         //
//     };
// }

// use petgraph::stable_graph::{
//     EdgeIndex,
//     NodeIndex,
// };

// pub struct Graph<T> {
//     g: petgraph::Graph<T, ()>,
// }

// impl<T> Graph<T> {
//     pub fn new() -> Self {
//         Self { g: <_>::default() }
//     }

//     pub fn insert(&mut self, w: T) -> NodeIndex {
//         self.g.add_node(w)
//     }

//     pub fn batch_insert(&mut self, edges: &[(T, T)])
//     where
//         T: Eq,
//     {
//         //
//     }

//     pub fn remove(&mut self, i: NodeIndex) -> Option<T> {
//         self.g.remove_node(i)
//     }

//     pub fn connect(
//         &mut self,
//         from: NodeIndex,
//         to: NodeIndex,
//     ) -> EdgeIndex {
//         self.g.add_edge(from, to, ())
//     }

//     pub fn disconnect(
//         &mut self,
//         e: EdgeIndex,
//     ) -> Option<()> {
//         self.g.remove_edge(e)
//     }

//     pub fn neighbors(&self) {
//         // self.g.neighbors(a)
//     }
// }
