pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Node {
                pub value: String,
                attrs: HashMap<String, &'static str>
            }
            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_string(),
                        attrs: HashMap::<String, &'static str>::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &'static str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1);
                    }
                    self
                }
                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).cloned()
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Edge {
                attrs: HashMap<String, String>
            }
            impl Edge {
                pub fn new(a : &str, b : &str) -> Self {
                    Edge{
                        attrs: HashMap::<String, String>::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }

    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new()
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }


        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs.iter() {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        pub fn get_node(self, node: &str) -> Result<Node, &str> {
            self.nodes.iter().find(|x| x.value == node.to_string()).ok_or("node must be stored").map(|x| x.clone())
        }

    }
}
