use crate::{
    core::{EdgeSampler, Node},
    preset::samplers::{BinaryMatrix, WeightedMatrix},
};

#[derive(Debug, Default)]
pub struct MatrixEdgeSampler;

impl<N> EdgeSampler<N, BinaryMatrix> for MatrixEdgeSampler
where
    N: Node<Key = u32>,
{
    type EdgeCandidate = (u32, u32);

    fn with_node(&self, node: &N, context: &BinaryMatrix) -> Vec<Self::EdgeCandidate> {
        if node.id() as usize >= context.len() {
            return vec![];
        }

        context[node.id() as usize]
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| (node.id(), i as u32))
            .collect()
    }
}

impl<N> EdgeSampler<N, WeightedMatrix> for MatrixEdgeSampler
where
    N: Node<Key = u32>,
{
    type EdgeCandidate = (u32, u32, f64);

    fn with_node(&self, node: &N, context: &WeightedMatrix) -> Vec<Self::EdgeCandidate> {
        if node.id() as usize >= context.len() {
            return vec![];
        }

        context[node.id() as usize]
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, v)| (node.id(), i as u32, v.unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{EdgeSampler, Node},
        testing::mock_node,
    };

    use super::MatrixEdgeSampler;

    #[test]
    fn sample_edges_from_binary_matrix() {
        let sampler = MatrixEdgeSampler::default();
        let domain = vec![
            vec![false, true, false], // 0->1
            vec![true, false, true],  // 1->0, 1->2
            vec![false, true, false], // 2->1
        ];

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
    fn sample_edges_from_weighted_matrix() {
        let sampler = MatrixEdgeSampler::default();
        let domain = vec![
            vec![None, Some(0.0), None],      // 0->1  0.0
            vec![Some(4.0), None, Some(2.0)], // 1->0  4.0, 1->2 2.0
            vec![None, Some(-1.0), None],     // 2->1 -1.0
        ];

        let mut candidates = sampler.with_node(&mock_node::<u32, ()>(0), &domain);
        assert_eq!(1, candidates.len());
        assert_eq!(0, candidates[0].0);
        assert_eq!(1, candidates[0].1);
        assert_eq!(0.0, candidates[0].2);

        candidates = sampler.with_node(&mock_node::<u32, ()>(1), &domain);
        assert_eq!(2, candidates.len());
        assert!(
            candidates
                .iter()
                .any(|c| c.0 == 1 && c.1 == 0 && c.2 == 4.0)
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
        assert_eq!(-1.0, candidates[0].2);
    }
}
