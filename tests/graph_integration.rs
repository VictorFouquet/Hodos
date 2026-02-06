mod graph_integration {
    use hodos::{
        core::{Edge, Frontier, Graph, Traverse, Visitor},
        preset::{BaseGraph, EmptyNode, Queue, UnweightedEdge},
    };

    #[test]
    fn traversal_populates_frontier_with_allowed_nodes() {
        // Graph is [(0->1), (0->2)]
        let mut graph = BaseGraph::new();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i));
            }
        }
        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

        assert!(!frontier.is_empty());
    }

    #[test]
    fn traversal_ends_when_frontier_is_empty() {
        // Graph has a single node and no edge
        let mut graph = BaseGraph::<_, UnweightedEdge<u32>>::new();
        graph.add_node(EmptyNode::new(0));

        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut NeverTerminateVisitor);

        assert!(frontier.is_empty());
    }

    #[test]
    fn traversal_ends_when_visitor_decides() {
        // Graph has a single node and no edge
        let mut graph = BaseGraph::new();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i));
            }
        }

        let mut frontier = Queue::new();

        graph.traverse(0, &mut frontier, &mut TerminateFirstVisitor);

        assert!(!frontier.is_empty());
    }

    #[test]
    fn traversal_uses_visitor_to_get_the_nodes_to_push() {
        // Graph is [(0->1), (0->2)]
        let mut graph = BaseGraph::new();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i));
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
        let mut graph = BaseGraph::new();
        for i in 0..3 {
            graph.add_node(EmptyNode::new(i));
            if i != 0 {
                graph.add_edge(UnweightedEdge::new(0, i));
            }
        }

        let mut visitor = LoopCountVisitor { count: 0 };
        graph.traverse(0, &mut Queue::new(), &mut visitor);
        assert_eq!(visitor.count, 1);
    }

    struct TerminateFirstVisitor;
    impl<Ctx: Graph> Visitor<Ctx> for TerminateFirstVisitor {
        fn should_explore(&mut self, _from: Ctx::Key, _to: Ctx::Key, _context: &Ctx) -> bool {
            true
        }

        fn next_to_explore(
            &mut self,
            node_id: <Ctx as Graph>::Key,
            context: &Ctx,
        ) -> Vec<(<Ctx as Graph>::Key, f64)> {
            context
                .get_edges_from(node_id)
                .iter()
                .map(|e| (e.to(), 0.0))
                .collect()
        }

        fn visit(&mut self, _node_id: Ctx::Key, _context: &Ctx) {}

        fn should_stop(&self, _node_id: Ctx::Key, _context: &Ctx) -> bool {
            true
        }
    }

    struct NeverTerminateVisitor;
    impl<Ctx: Graph> Visitor<Ctx> for NeverTerminateVisitor {
        fn should_explore(&mut self, _from: Ctx::Key, _to: Ctx::Key, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: Ctx::Key, _context: &Ctx) {}

        fn should_stop(&self, _node_id: Ctx::Key, _context: &Ctx) -> bool {
            false
        }
    }

    struct ExploreAllVisitor;
    impl<Ctx: Graph> Visitor<Ctx> for ExploreAllVisitor {
        fn should_explore(&mut self, _from: Ctx::Key, _to: Ctx::Key, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: Ctx::Key, _context: &Ctx) {}

        fn next_to_explore(
            &mut self,
            node_id: <Ctx as Graph>::Key,
            context: &Ctx,
        ) -> Vec<(<Ctx as Graph>::Key, f64)> {
            context
                .get_edges_from(node_id)
                .iter()
                .map(|e| (e.to(), 0.0))
                .collect()
        }

        fn should_stop(&self, _node_id: Ctx::Key, _context: &Ctx) -> bool {
            true
        }
    }

    struct ExploreNoneVisitor;
    impl<Ctx: Graph> Visitor<Ctx> for ExploreNoneVisitor {
        fn should_explore(&mut self, _from: Ctx::Key, _to: Ctx::Key, _context: &Ctx) -> bool {
            false
        }

        fn visit(&mut self, _node_id: Ctx::Key, _context: &Ctx) {}

        fn should_stop(&self, _node_id: Ctx::Key, _context: &Ctx) -> bool {
            true
        }
    }

    struct LoopCountVisitor {
        pub count: u32,
    }
    impl<Ctx: Graph> Visitor<Ctx> for LoopCountVisitor {
        fn should_explore(&mut self, _from: Ctx::Key, _to: Ctx::Key, _context: &Ctx) -> bool {
            true
        }

        fn visit(&mut self, _node_id: Ctx::Key, _context: &Ctx) {
            self.count += 1;
        }

        fn should_stop(&self, _node_id: Ctx::Key, _context: &Ctx) -> bool {
            self.count == 3
        }
    }
}
