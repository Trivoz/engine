//! Contains a pre-defined cube mesh - defined as a unit
//! cube as it is scaled (and projected) seperately

use crate::{Mesh, Triangle, Vector3D};

/// A function that is used to get a pre-defined cube mesh.
///
/// # Returns
/// * `Mesh` - The pre-defined cube mesh
///
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
