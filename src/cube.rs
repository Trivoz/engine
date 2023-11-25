use crate::{Mesh, Triangle, Vector3D};

pub fn get_cube_mesh() -> Mesh {
    let mesh_vertices = vec![
        // SOUTH
        [
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 1.0, 0.0),
        ],
        [
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(1.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
        ],
        // EAST
        [
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(1.0, 1.0, 0.0),
            Vector3D::new(1.0, 1.0, 1.0),
        ],
        [
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(1.0, 1.0, 1.0),
            Vector3D::new(1.0, 0.0, 1.0),
        ],
        // NORTH
        [
            Vector3D::new(1.0, 0.0, 1.0),
            Vector3D::new(1.0, 1.0, 1.0),
            Vector3D::new(0.0, 1.0, 1.0),
        ],
        [
            Vector3D::new(1.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 1.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ],
        // WEST
        [
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ],
        [
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, 0.0, 0.0),
        ],
        // TOP
        [
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, 1.0, 1.0),
            Vector3D::new(1.0, 1.0, 1.0),
        ],
        [
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 1.0, 1.0),
            Vector3D::new(1.0, 1.0, 0.0),
        ],
        // BOTTOM
        [
            Vector3D::new(1.0, 0.0, 1.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 0.0, 0.0),
        ],
        [
            Vector3D::new(1.0, 0.0, 1.0),
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
        ],
    ];

    Mesh::new(
        mesh_vertices
            .iter()
            .map(|triangle| Triangle::new(triangle[0], triangle[1], triangle[2]))
            .collect(),
    )
}
