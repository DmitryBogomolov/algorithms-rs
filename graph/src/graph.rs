pub trait Graph {
    type VertexIter<'a>: Iterator<Item = usize> + ExactSizeIterator
    where
        Self: 'a;

    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn adjacent_vertices(&self, vertex: usize) -> Self::VertexIter<'_>;
}
