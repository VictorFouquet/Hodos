use crate::{
    core::{EdgeSampler, Node},
    preset::samplers::{AdjacencyList, WeightedAdjacencyList},
};

#[derive(Debug, Default)]
pub struct ListEdgeSampler;

impl<N> EdgeSampler<N, AdjacencyList> for ListEdgeSampler
where
    N: Node<Key = u32>,
{
    type EdgeCandidate = (u32, u32);

    fn with_node(&self, node: &N, context: &AdjacencyList) -> Vec<Self::EdgeCandidate> {
        if node.id() as usize >= context.len() {
            return vec![];
        }

        context[node.id() as usize]
            .iter()
            .map(|c| (node.id(), *c))
            .collect()
    }
}

impl<N> EdgeSampler<N, WeightedAdjacencyList> for ListEdgeSampler
where
    N: Node<Key = u32>,
{
    type EdgeCandidate = (u32, u32, f64);

    fn with_node(&self, node: &N, context: &WeightedAdjacencyList) -> Vec<Self::EdgeCandidate> {
        if node.id() as usize >= context.len() {
            return vec![];
        }

        context[node.id() as usize]
            .iter()
            .map(|c| (node.id(), c.0, c.1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::EdgeSampler, testing::mock_node};

    use super::ListEdgeSampler;

    #[test]
    fn get_candidate_edges_from_uniform_adjacency_list() {
        let sampler = ListEdgeSampler::default();

        let domain = vec![vec![1], vec![0, 2], vec![1]];

        let mut candidates = sampler.with_node(&mock_node::<u32, ()>(0), &domain);
        assert_eq!(1, candidates.len());
        assert_eq!(0, candidates[0].0);
        assert_eq!(1, candidates[0].1);

        candidates = sampler.with_node(&mock_node::<u32, ()>(1), &domain);
        assert_eq!(2, candidates.len());
        assert!(candidates.iter().any(|c| c.0 == 1 && c.1 == 0));
        assert!(candidates.iter().any(|c| c.0 == 1 && c.1 == 2));

        candidates = sampler.with_node(&mock_node::<u32, ()>(2), &domain);
        assert_eq!(1, candidates.len());
        assert_eq!(2, candidates[0].0);
        assert_eq!(1, candidates[0].1);
    }

    #[test]
    fn get_candidate_edges_from_weighted_adjacency_list() {
        let sampler = ListEdgeSampler::default();

        let domain = vec![vec![(1, 1.0)], vec![(0, 1.0), (2, 2.0)], vec![(1, 2.0)]];

        let mut candidates = sampler.with_node(&mock_node::<u32, ()>(0), &domain);
        assert_eq!(1, candidates.len());
        assert_eq!(0, candidates[0].0);
        assert_eq!(1, candidates[0].1);
        assert_eq!(1.0, candidates[0].2);

        candidates = sampler.with_node(&mock_node::<u32, ()>(1), &domain);
        assert_eq!(2, candidates.len());
        assert!(
            candidates
                .iter()
                .any(|c| c.0 == 1 && c.1 == 0 && c.2 == 1.0)
        );
        assert!(
            candidates
                .iter()
                .any(|c| c.0 == 1 && c.1 == 2 && c.2 == 2.0)
        );

        candidates = sampler.with_node(&mock_node::<u32, ()>(2), &domain);
        assert_eq!(1, candidates.len());
        assert_eq!(2, candidates[0].0);
        assert_eq!(1, candidates[0].1);
        assert_eq!(2.0, candidates[0].2);
    }
}
