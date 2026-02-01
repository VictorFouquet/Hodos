pub trait TrackParent {
    fn get_parent(&self, _node_id: u32) -> Option<u32> {
        None
    }
}
