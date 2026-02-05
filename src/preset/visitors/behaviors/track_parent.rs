pub trait TrackParent<K> {
    fn get_parent(&self, _node_id: K) -> Option<K> {
        None
    }
}
