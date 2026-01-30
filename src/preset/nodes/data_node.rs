use crate::core::{HasData, Node};

/// A graph node that can contain associated data.
///
/// This is a basic implementation suitable for any graph where nodes
/// need an identifier and additional metadata.
///
/// # Examples
///
/// ```
/// use hodos::preset::nodes::DataNode;
/// use hodos::core::{HasData, Node};
///
/// #[derive(Clone, Copy)]
/// struct Point {
///     pub x: u32,
///     pub y: u32,
/// };
///
/// let mut node = DataNode::new(42, Point { x: 1, y: 2 });
/// assert_eq!(node.id(), 42);
/// assert_eq!(node.data().x, 1);
/// assert_eq!(node.data().y, 2);
/// ```
#[derive(Debug, Default)]
pub struct DataNode<T> {
    id: u32,
    data: T,
}

impl<T> DataNode<T> {
    pub fn new(id: u32, data: T) -> Self {
        DataNode { id, data }
    }
}

impl<T: Copy + Clone> HasData for DataNode<T> {
    type Data = T;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn set_data(&mut self, data: Self::Data) {
        self.data = data;
    }
}

impl<T: Clone> Node for DataNode<T> {
    fn id(&self) -> u32 {
        self.id
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
    fn set_data_updates_node_data() {
        let data1 = TestData {
            val1: 0,
            val2: false,
        };
        let data2 = TestData {
            val1: 1,
            val2: true,
        };

        let mut node = DataNode::new(0, data1);
        assert_eq!(node.data(), &data1);

        node.set_data(data2);
        assert_eq!(node.data(), &data2);
    }
}
