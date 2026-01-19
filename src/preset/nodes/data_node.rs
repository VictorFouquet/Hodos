use crate::graph::Node;

/// A graph node that can contain data associated data.
///
/// This is a basic implementation suitable for any graph where nodes
/// need an identifier and additional metadata.
///
/// # Examples
///
/// ```
/// use hodos::preset::nodes::DataNode;
/// use hodos::graph::Node;
///
/// let mut node = DataNode::new(42, Some(true));
/// assert_eq!(node.id(), 42);
/// assert_eq!(node.data(), Some(&true));
/// node.set_data(&false);
/// assert_eq!(node.data(), Some(&false));
/// ```
#[derive(Debug, Default)]
pub struct DataNode<T> {
    id: u32,
    data: T,
}

impl<T: Clone> Node for DataNode<T> {
    type Data = T;

    fn new(id: u32, data: Option<Self::Data>) -> Self {
        DataNode {
            id,
            data: data.expect("Data node must be provided with data"),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
    fn data(&self) -> Option<&Self::Data> {
        Some(&self.data)
    }
    fn set_data(&mut self, data: &Self::Data) {
        self.data = data.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct TestData {
        val1: u32,
        val2: bool,
    }

    #[test]
    #[should_panic(expected = "Data node must be provided with data")]
    fn new_panics_without_data() {
        DataNode::<u32>::new(0, None);
    }

    #[test]
    fn set_data_updates_node_data() {
        let data1 = TestData {
            val1: 0,
            val2: false,
        };
        let data2 = TestData {
            val1: 1,
            val2: true,
        };

        let mut node = DataNode::new(0, Some(data1));
        assert_eq!(node.data(), Some(&data1));

        node.set_data(&data2);
        assert_eq!(node.data(), Some(&data2));
    }
}
