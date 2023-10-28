use super::Vertex;

// Vertex
static mut VERTEX_CACHE: Vec<Vec<Vertex>> = Vec::new();

pub fn alloc_vertex(vertex: Vec<Vertex>) -> usize {
    unsafe {
        VERTEX_CACHE.push(vertex);
        VERTEX_CACHE.len() - 1
    }
}

pub fn get_vertex(index: usize) -> &'static Vec<Vertex> {
    unsafe {
        VERTEX_CACHE.get(index).unwrap()
    }
}

pub fn clear_vertex() {
    unsafe {
        VERTEX_CACHE.clear();
    }
}

// Index
static mut INDEX_CACHE: Vec<Vec<u16>> = Vec::new();

pub fn alloc_index(index: Vec<u16>) -> usize {
    unsafe {
        INDEX_CACHE.push(index);
        INDEX_CACHE.len() - 1
    }
}

pub fn get_index(index: usize) -> &'static Vec<u16> {
    unsafe {
        INDEX_CACHE.get(index).unwrap()
    }
}

pub fn clear_index() {
    unsafe {
        INDEX_CACHE.clear();
    }
}