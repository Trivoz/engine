//! The main file of the engine

#![allow(unused)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod cube;

use core::fmt::{self, Display};
use warn;

/// A simple vector that is 3d which has 3 common components that represent each dimension.
#[derive(Debug, Clone)]
pub struct Vector3D {
    /// 'x' for the x-axis
    pub x: f32,
    /// 'y' for the y-axis
    pub y: f32,
    /// 'z' for the z-axis
    pub z: f32,
}

// Note that there is no function for this as it just returns a copy of the vector.
impl Copy for Vector3D {}

impl Vector3D {
    /// Create a new vector
    ///
    /// # Arguments
    /// * `x` - The x-axis component of the vector
    /// * `y` - The y-axis component of the vector
    /// * `z` - The z-axis component of the vector
    ///
    /// # Returns
    /// * `Vector3D` - The new vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Clone an instance of a vector
    ///
    /// # Returns
    /// * `Vector3D` - The cloned vector
    fn clone(&self) -> Vector3D {
        Vector3D {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

/// It is handy to have vectors already pre-defined if we don't want to type them out by hand.
impl Default for Vector3D {
    /// Create a default vector
    ///
    /// # Returns
    /// * `Vector3D` - The default vector
    fn default() -> Self {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// If we want to debug a vector (or multiple) its handy to print them out automatically
/// instead of by hand.
impl Display for Vector3D {
    /// Print the vector
    ///
    /// # Arguments
    /// * `f` - The formatter
    ///
    /// # Returns
    /// * `fmt::Result` - The result of the formatter
    ///
    /// # Examples
    /// ```
    /// let vector = Vector3D::new(1.0, 1.0, 1.0);
    /// println!("{}", vector);
    ///
    /// // Output:
    /// // X: 1
    /// // Y: 1
    /// // Z: 1
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X: {}\nY :{}\nZ: {}\n", self.x, self.y, self.z)
    }
}

/// There are a plethora of ways to group vertices together.
/// I am choosing to group them into triangles to comprise a mesh,
/// instead of other 2d primitives such as a square. This is because
/// there are much more expansive optimisation algorithms that can be applied
/// to a group of triangles, as they ultimately require less processing power.
#[derive(Debug)]
pub struct Triangle {
    /// `a` - The first vector in the triangle
    pub a: Vector3D,
    /// `b` - The second vector in the triangle
    pub b: Vector3D,
    /// `c` - The third vector in the triangle
    pub c: Vector3D,
}

impl Clone for Triangle {
    /// Clone the triangle
    ///
    /// # Returns
    /// * `Triangle` - The cloned triangle
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
        }
    }
}

impl Default for Triangle {
    /// Create a default triangle with all values set to 0
    ///
    /// # Returns
    /// * `Triangle` - The default triangle
    fn default() -> Self {
        Self {
            a: Vector3D::default(),
            b: Vector3D::default(),
            c: Vector3D::default(),
        }
    }
}

/// Matrix struct for the projection matrix and model matrix
pub struct Matrix {
    /// `mat` - The matrix itself
    pub mat: [[f32; 4]; 4],
}

impl Matrix {
    /// Create a new matrix
    ///
    /// # Arguments
    /// * `mat` - The matrix itself
    ///
    /// # Returns
    /// * `Matrix` - The new matrix
    pub fn new(mat: [[f32; 4]; 4]) -> Self {
        Self { mat }
    }
}

impl Clone for Matrix {
    /// Clone the matrix
    ///
    /// # Returns
    /// * `Matrix` - The cloned matrix
    fn clone(&self) -> Self {
        Self {
            mat: self.mat.clone(),
        }
    }
}

impl Default for Matrix {
    /// Create a default matrix with all values set to 0
    ///
    /// # Returns
    /// * `Matrix` - The default matrix
    fn default() -> Self {
        Self { mat: [[0.0; 4]; 4] }
    }
}

impl Triangle {
    /// This function is an alternative to a raw if statement since doing the alternative if
    /// statement would break the code under E0317 (if expressions with else evaluate to `()`)
    ///
    /// # Arguments
    /// * `mat` - The matrix to check
    ///
    /// # Returns
    /// * `Option<&'static str>` - The warning if the matrix is too big
    pub fn warn_triangle_size(mat: &[Vector3D; 3]) -> Option<&'static str> {
        if mat.len() > 3 {
            Some("Triangle is too big, consider splitting it up")
        } else {
            None
        }
    }

    /// Create a new triangle
    ///
    /// # Arguments
    /// * `a` - The first vector in the triangle
    /// * `b` - The second vector in the triangle
    /// * `c` - The third vector in the triangle
    ///
    /// # Returns
    /// * `Triangle` - The new triangle
    pub fn new(a: Vector3D, b: Vector3D, c: Vector3D) -> Self {
        Self { a, b, c }
    }

    /// Draw the triangle on the screen
    ///
    /// # Arguments
    /// * `canvas` - The canvas to draw the triangle on
    ///
    /// # Returns
    /// * `()` - Nothing
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> () {
        canvas.draw_line(
            sdl2::rect::Point::new(self.a.x as i32, self.a.y as i32),
            sdl2::rect::Point::new(self.b.x as i32, self.b.y as i32),
        );
        canvas.draw_line(
            sdl2::rect::Point::new(self.b.x as i32, self.b.y as i32),
            sdl2::rect::Point::new(self.c.x as i32, self.c.y as i32),
        );
        canvas.draw_line(
            sdl2::rect::Point::new(self.c.x as i32, self.c.y as i32),
            sdl2::rect::Point::new(self.a.x as i32, self.a.y as i32),
        );
    }
}

/// For convenience purposes, we are also going to create a mesh.
/// This is not only for convenience but also for optimization because
/// it lets us store vector positions but not process them until we need them,
/// following the idea of RAII (resource allocation is initialization)
pub struct Mesh {
    /// 'mat' stands for matrix
    pub mat: Vec<Triangle>,
}

impl<W> warn::Warn<W> for Mesh {
    /// Print a warning to the console
    ///
    /// # Arguments
    /// * `warning` - The warning to print
    ///
    /// # Returns
    /// * `()` - Nothing
    fn warn(&mut self, warning: W) -> () {
        println!("{}", stringify!(warning))
    }
}

/// We should implement functionality for the Mesh, because we want to set certain buffers,
/// those being resource allocation warnings if the mesh becomes too big. If that happens,
/// it should send a warning, advising that the mesh be split up.
///
/// This is only for scaling purposes i.e. if we want to implement functionality for the engine
/// wherein a text file containing various 3d points can be loaded in.
impl Mesh {
    /// The amount of vectors that a single mesh can contain before being a burden on the computer
    /// memory. At which point, send a warning/advisement that the mesh should be split up.
    const VECTOR_LIMIT: usize = 50;

    /// This function is an alternative to a raw if statement since doing the alternative if
    /// statement would break the code under E0317 (if expressions with else evaluate to `()`)
    ///
    /// # Arguments
    /// * `mat` - The matrix to check
    ///
    /// # Returns
    /// * `Option<&'static str>` - The warning if the matrix is too big
    pub fn warn_mesh_size(mat: &Vec<Triangle>) -> Option<&'static str> {
        if mat.len() > Mesh::VECTOR_LIMIT {
            Some("Mesh is too big, consider splitting it up")
        } else {
            None
        }
    }

    /// Draw the mesh on the screen
    ///
    /// # Arguments
    /// * `canvas` - The canvas to draw the mesh on
    ///
    /// # Returns
    /// * `()` - Nothing
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> () {
        unimplemented!()
    }

    /// Create a new mesh
    ///
    /// # Arguments
    /// * `mat` - The matrix to create the mesh from
    ///
    /// # Returns
    /// * `Mesh` - The new mesh
    fn new(mat: Vec<Triangle>) -> Self {
        Self::warn_mesh_size(&mat);
        Self { mat }
    }
}

/// Multiply a vector by a Matrix
///
/// # Arguments
///
/// * `i` - The input vector
/// * `o` - The output vector
/// * `m` - The matrix to multiply the vector by
///
/// # Returns
/// * `o` - The output vector
///
/// # Examples
/// ```
/// let mut input_vector = Vector3D::new(1.0, 1.0, 1.0);
/// let mut output_vector = Vector3D::default();
/// let matrix = Matrix::default();
/// multiply_matrix_vector(&input_vector, &mut output_vector, &matrix);
///
/// assert_eq!(output_vector, Vector3D::default());
/// ```
pub fn multiply_matrix_vector<'a>(
    i: &'a Vector3D,
    o: &'a mut Vector3D,
    m: &'a Matrix,
) -> &'a mut Vector3D {
    o.x = i.x * m.mat[0][0] + i.y * m.mat[1][0] + i.z * m.mat[2][0] + m.mat[3][0];
    o.y = i.x * m.mat[0][1] + i.y * m.mat[1][1] + i.z * m.mat[2][1] + m.mat[3][1];
    o.z = i.x * m.mat[0][2] + i.y * m.mat[1][2] + i.z * m.mat[2][2] + m.mat[3][2];

    let w: f32 = i.x * m.mat[0][3] + i.y * m.mat[1][3] + i.z * m.mat[2][3] + m.mat[3][3];

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }

    o
}

/// The main function of the engine (also runs the game loop)
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screen_size = video_subsystem.display_bounds(0).unwrap();

    let display_width = screen_size.width() as f32 / 1.5;
    let display_height = screen_size.height() as f32 / 1.5;

    let field_of_view: f32 = 90.0;
    let near_plane: f32 = 0.1;
    let far_plane: f32 = 1000.0;
    let aspect_ratio: f32 = display_height / display_width;
    let scaling_factor: f32 = 1.0 / (field_of_view / 2.0).tan();
    let mut projection_matrix: Matrix = Matrix::default();
    projection_matrix.mat = [
        [aspect_ratio * scaling_factor, 0.0, 0.0, 0.0],
        [0.0, scaling_factor, 0.0, 0.0],
        [0.0, 0.0, far_plane / (far_plane - near_plane), 1.0],
        [
            0.0,
            0.0,
            (-far_plane * near_plane) / (far_plane - near_plane),
            0.0,
        ],
    ];

    let mut model_matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0], // X
        [0.0, 1.0, 0.0, 0.0], // Y
        [0.0, 0.0, 1.0, 0.0], // Z
        [0.0, 0.0, 0.0, 1.0], // W
    ];

    let mut cube_mesh = cube::get_cube_mesh();

    projection_matrix.mat[0][0] = aspect_ratio * scaling_factor;
    projection_matrix.mat[1][1] = scaling_factor;
    projection_matrix.mat[2][2] = far_plane / (far_plane - near_plane);
    projection_matrix.mat[3][2] = (-far_plane * near_plane) / (far_plane - near_plane);
    projection_matrix.mat[2][3] = 1.0;
    projection_matrix.mat[3][3] = 0.0;

    let window: sdl2::video::Window = video_subsystem
        .window(
            "rust-sdl2 demo",
            display_width as u32,
            display_height as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let black: Color = Color::RGB(0, 0, 0);
    let white: Color = Color::RGB(255, 255, 255);
    let background_color: Color = black;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let aspect_ratio = display_height as f32 / display_width as f32;

    'running: loop {
        canvas.set_draw_color(background_color);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Set the drawing color to white
        canvas.set_draw_color(white);

        // Draw the cube
        for triangle in cube_mesh.mat.iter_mut() {
            let mut tri_projected: Triangle = Triangle::default();
            let mut tri_translated: Triangle = triangle.clone();

            // Add depth to the triangle
            tri_translated.a.z = triangle.a.z + 3.0;
            tri_translated.b.z = triangle.b.z + 3.0;
            tri_translated.c.z = triangle.c.z + 3.0;

            multiply_matrix_vector(&tri_translated.a, &mut tri_projected.a, &projection_matrix);
            multiply_matrix_vector(&tri_translated.b, &mut tri_projected.b, &projection_matrix);
            multiply_matrix_vector(&tri_translated.c, &mut tri_projected.c, &projection_matrix);

            // Scale into view
            tri_projected.a.x += 1.0;
            tri_projected.a.y += 1.0;
            tri_projected.b.x += 1.0;
            tri_projected.b.y += 1.0;
            tri_projected.c.x += 1.0;
            tri_projected.c.y += 1.0;

            tri_projected.a.x *= 0.5 * display_width;
            tri_projected.a.y *= 0.5 * display_height;
            tri_projected.b.x *= 0.5 * display_width;
            tri_projected.b.y *= 0.5 * display_height;
            tri_projected.c.x *= 0.5 * display_width;
            tri_projected.c.y *= 0.5 * display_height;

            tri_projected.draw(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
