use crate::core::NodeSampler;

#[derive(Debug, Default)]
pub struct VecNodeSampler {
    current_id: usize,
}

impl<T> NodeSampler<Vec<T>> for VecNodeSampler {
    type NodeCandidate = u32;
    fn next(&mut self, context: &Vec<T>) -> Option<Vec<Self::NodeCandidate>> {
        if self.current_id >= context.len() {
            return None;
        }
        let candidate = self.current_id as u32;
        self.current_id += 1;
        Some(vec![candidate])
    }
}

#[cfg(test)]
mod tests {
    use crate::core::NodeSampler;

    use super::VecNodeSampler;

    #[test]
    fn read_nodes_with_internal_counter() {
        let mut sampler = VecNodeSampler::default();

        assert_eq!(0, sampler.current_id);

        let domain = vec![vec![1], vec![0, 2], vec![1]];

        let mut candidate = sampler.next(&domain).unwrap();

        assert_eq!(1, candidate.len());
        assert_eq!(0, candidate[0]);

        candidate = sampler.next(&domain).unwrap();

        assert_eq!(1, candidate.len());
        assert_eq!(1, candidate[0]);

        candidate = sampler.next(&domain).unwrap();

        assert_eq!(1, candidate.len());
        assert_eq!(2, candidate[0]);
    }

    #[test]
    fn stops_when_reaching_end_of_list() {
        let mut sampler = VecNodeSampler::default();
        let domain = vec![vec![1], vec![0, 2], vec![1]];

        assert!(sampler.next(&domain).is_some());
        assert!(sampler.next(&domain).is_some());
        assert!(sampler.next(&domain).is_some());

        assert!(sampler.next(&domain).is_none());
    }
}
