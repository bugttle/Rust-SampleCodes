// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/associated-types.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Associated Types

    // trait Graph<N, E> {
    //     fn has_edge(&self, &N, &N) -> bool;
    //     fn edges(&self, &N) -> Vec<E>;
    //     // Etc.
    // }
    // fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { ... }

    ////////////////////////////////////////////////////////////////////////////////
    // Defining associated types 

    // trait Graph {
    //     type N;
    //     type E;
    //
    //     fn has_edge(&self, &Self::N, &Self::N) -> bool;
    //     fn edges(&self, &Self::N) -> Vec<Self::E>;
    //     // Etc.
    // }
    // fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 { ... }

    // use std::fmt;
    //
    // trait Graph {
    //     type N: fmt::Display;
    //     type E;
    //
    //     fn has_edge(&self, &Self::N, &Self::N) -> bool;
    //     fn edges(&self, &Self::N) -> Vec<Self::E>;
    //     // Etc.
    // }

    ////////////////////////////////////////////////////////////////////////////////
    // Implementing associated types

    trait Graph {
        type N;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        // Etc.
    }

    struct Node;
    struct Edge;
    struct MyGraph;

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            true
        }

        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Trait objects with associated types

    let graph = MyGraph;

    // error[E0191]: the value of the associated type `E` (from the trait `main::Graph`) must be specified
    // let obj = Box::new(graph) as Box<Graph>;

    let obj = Box::new(graph) as Box<Graph<N = Node, E = Edge>>;
}
