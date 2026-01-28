mod graph_integration {
    use hodos::{
        frontier::{Frontier, Queue},
        graph::{Edge, Graph, Node},
        preset::{EmptyNode, UnweightedEdge},
        strategy::Visitor,
    };

    #[test]
    fn traversal_populates_frontier_with_allowed_nodes() {
        // Graph is [(0->1), (0->2)]
        let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i, None));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i, None));
            }
        }
        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

        assert!(!frontier.is_empty());
    }

    #[test]
    fn traversal_ends_when_frontier_is_empty() {
        // Graph has a single node and no edge
        let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
        graph.add_node(EmptyNode::new(0, None));

        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut NeverTerminateVisitor);

        assert!(frontier.is_empty());
    }

    #[test]
    fn traversal_ends_when_visitor_decides() {
        // Graph has a single node and no edge
        let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i, None));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i, None));
            }
        }

        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

        assert!(!frontier.is_empty());
    }

    #[test]
    fn traversal_uses_visitor_to_handle_push_decision() {
        // Graph is [(0->1), (0->2)]
        let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i, None));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i, None));
            }
        }

        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut ExploreNoneVisitor);
        assert!(frontier.is_empty());

        graph.traverse(0, &mut frontier, &mut ExploreAllVisitor);
        assert!(!frontier.is_empty());
    }

    #[test]
    fn traversal_lets_visitor_visit_one_node_per_iteration() {
        // Graph is [(0->1), (0->2)]
        let mut graph: Graph<EmptyNode, UnweightedEdge> = Graph::default();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i, None));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i, None));
            }
        }

        let mut visitor = LoopCountVisitor { count: 0 };
        graph.traverse(0, &mut Queue::new(), &mut visitor);
        assert_eq!(visitor.count, 3);
    }

    struct TerminateFirstVisitor;
    impl<Ctx> Visitor<Ctx> for TerminateFirstVisitor {
        fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

        fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
            true
        }
    }

    struct NeverTerminateVisitor;
    impl<Ctx> Visitor<Ctx> for NeverTerminateVisitor {
        fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

        fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
            false
        }
    }

    struct ExploreAllVisitor;
    impl<Ctx> Visitor<Ctx> for ExploreAllVisitor {
        fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

        fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
            true
        }
    }

    struct ExploreNoneVisitor;
    impl<Ctx> Visitor<Ctx> for ExploreNoneVisitor {
        fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
            false
        }

        fn visit(&mut self, _node_id: u32, _context: &Ctx) {}

        fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
            true
        }
    }

    struct LoopCountVisitor {
        pub count: u32,
    }
    impl<Ctx> Visitor<Ctx> for LoopCountVisitor {
        fn should_explore(&mut self, _from: u32, _to: u32, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: u32, _context: &Ctx) {
            self.count += 1;
        }

        fn should_stop(&self, _node_id: u32, _context: &Ctx) -> bool {
            self.count == 3
        }
    }
}
