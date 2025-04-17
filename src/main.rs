// macroquad provides simple graphics and game development functionality
// HashSet is used for efficient edge deduplication
use macroquad::prelude::*;
use std::collections::HashSet;

// Convert HSV (Hue, Saturation, Value) color to RGB (Red, Green, Blue) color
// HSV is often more intuitive for color manipulation than RGB
// Parameters:
//   h: hue (0-1) - represents the color (0=red, 1/3=green, 2/3=blue)
//   s: saturation (0-1) - how intense the color is (0=gray, 1=full color)
//   v: value/brightness (0-1) - how bright the color is (0=black, 1=full brightness)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    // Convert hue to a value between 0 and 6 for easier calculation
    let i = (h * 6.0).floor();
    // Get the fractional part of the hue
    let f = h * 6.0 - i;
    // Calculate intermediate values for RGB conversion
    let p = v * (1.0 - s);      // Minimum RGB component
    let q = v * (1.0 - f * s);  // Intermediate RGB component
    let t = v * (1.0 - (1.0 - f) * s);  // Another intermediate RGB component

    // Convert to RGB based on which segment of the color wheel we're in
    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),  // Red to Yellow
        1 => (q, v, p),  // Yellow to Green
        2 => (p, v, t),  // Green to Cyan
        3 => (p, q, v),  // Cyan to Blue
        4 => (t, p, v),  // Blue to Magenta
        5 => (v, p, q),  // Magenta to Red
        _ => (0.0, 0.0, 0.0),  // Fallback (should never happen)
    };
    // Create and return the final color with full opacity
    Color::new(r, g, b, 1.0)
}

// Define a structure to represent a point in 3D space
// This is used to store the vertices of our dodecahedron
struct Point3D {
    x: f32,  // X coordinate
    y: f32,  // Y coordinate
    z: f32,  // Z coordinate
}

// Implementation of methods for the Point3D structure
impl Point3D {
    // Constructor to create a new 3D point
    // Parameters:
    //   x, y, z: coordinates in 3D space
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Rotate the point around the X axis
    // This is done using the standard 3D rotation matrix for X-axis rotation
    // Parameters:
    //   angle: rotation angle in radians
    fn rotate_x(&self, angle: f32) -> Self {
        let cos = angle.cos();  // Pre-calculate cosine
        let sin = angle.sin();  // Pre-calculate sine
        Self {
            x: self.x,  // X coordinate remains unchanged
            y: self.y * cos - self.z * sin,  // New Y after rotation
            z: self.y * sin + self.z * cos,  // New Z after rotation
        }
    }

    // Rotate the point around the Y axis
    // Similar to rotate_x but using Y-axis rotation matrix
    fn rotate_y(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos + self.z * sin,
            y: self.y,  // Y coordinate remains unchanged
            z: -self.x * sin + self.z * cos,
        }
    }

    // Rotate the point around the Z axis
    // Similar to previous rotations but using Z-axis rotation matrix
    fn rotate_z(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,  // Z coordinate remains unchanged
        }
    }

    // Project a 3D point onto a 2D screen
    // This implements a simple perspective projection
    // The result is a 2D point that can be drawn on screen
    fn project(&self) -> Vec2 {
        let scale = 200.0;  // Scale factor to control the size of the projection
        let z = self.z + 5.0;  // Add distance to prevent division by zero
        // Calculate screen coordinates with perspective
        vec2(
            self.x * scale / z + screen_width() / 2.0,   // Center horizontally
            self.y * scale / z + screen_height() / 2.0,  // Center vertically
        )
    }
}

// Generate a list of unique edges from the faces of the dodecahedron
// This prevents drawing the same edge multiple times, which would be inefficient
// Parameters:
//   faces: array of face definitions, where each face is defined by 5 vertex indices
fn get_unique_edges(faces: &[[usize; 5]]) -> Vec<(usize, usize)> {
    let mut edge_set = HashSet::new();  // Used to track unique edges
    let mut edges = Vec::new();         // Store the final list of unique edges

    // Iterate through each face of the dodecahedron
    for face in faces {
        // For each vertex in the face
        for i in 0..5 {
            let a = face[i];                    // Current vertex
            let b = face[(i + 1) % 5];          // Next vertex (wrapping around)
            // Store edge in consistent order (smaller index first)
            // This ensures (1,2) and (2,1) are treated as the same edge
            let edge = if a < b { (a, b) } else { (b, a) };

            // If this is a new edge, add it to our list
            if edge_set.insert(edge) {
                edges.push(edge);
            }
        }
    }

    edges  // Return the list of unique edges
}

// Main program entry point
#[macroquad::main("Color Coded")]
async fn main() {
    // Animation parameters
    let mut h = 0.0;        // Hue value for color cycling (0-1)
    let mut angle_x = 0.0;  // Current rotation angle around X axis
    let mut angle_y = 0.0;  // Current rotation angle around Y axis
    let mut angle_z = 0.0;  // Current rotation angle around Z axis

    // Golden ratio constant (φ = (1 + √5)/2)
    // This is used in the construction of regular dodecahedrons
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0;

    // Define the 20 vertices of a regular dodecahedron
    // A dodecahedron has 20 vertices, each defined by 3D coordinates
    // The coordinates are based on the golden ratio for perfect regularity
    let vertices = [
        // First 8 vertices form a cube
        Point3D::new(1.0, 1.0, 1.0),      // Front-top-right
        Point3D::new(1.0, 1.0, -1.0),     // Front-top-left
        Point3D::new(1.0, -1.0, 1.0),     // Front-bottom-right
        Point3D::new(1.0, -1.0, -1.0),    // Front-bottom-left
        Point3D::new(-1.0, 1.0, 1.0),     // Back-top-right
        Point3D::new(-1.0, 1.0, -1.0),    // Back-top-left
        Point3D::new(-1.0, -1.0, 1.0),    // Back-bottom-right
        Point3D::new(-1.0, -1.0, -1.0),   // Back-bottom-left
        // Additional vertices to complete the dodecahedron
        Point3D::new(0.0, 1.0/phi, phi),  // Top-front
        Point3D::new(0.0, 1.0/phi, -phi), // Top-back
        Point3D::new(0.0, -1.0/phi, phi), // Bottom-front
        Point3D::new(0.0, -1.0/phi, -phi),// Bottom-back
        Point3D::new(1.0/phi, phi, 0.0),  // Right-top
        Point3D::new(1.0/phi, -phi, 0.0), // Right-bottom
        Point3D::new(-1.0/phi, phi, 0.0), // Left-top
        Point3D::new(-1.0/phi, -phi, 0.0),// Left-bottom
        Point3D::new(phi, 0.0, 1.0/phi),  // Front-right
        Point3D::new(phi, 0.0, -1.0/phi), // Front-left
        Point3D::new(-phi, 0.0, 1.0/phi), // Back-right
        Point3D::new(-phi, 0.0, -1.0/phi),// Back-left
    ];

    // Define the 12 faces of the dodecahedron
    // Each face is a pentagon defined by 5 vertex indices
    // The indices refer to positions in the vertices array above
    let faces = [
        [0, 8, 10, 2, 16],   // Front face
        [0, 16, 17, 1, 8],   // Top face
        [0, 12, 4, 14, 8],   // Right face
        [8, 14, 5, 9, 1],    // Top-back face
        [16, 17, 3, 13, 2],  // Front-bottom face
        [1, 9, 11, 3, 17],   // Left face
        [2, 10, 6, 15, 13],  // Bottom face
        [3, 11, 7, 15, 13],  // Back face
        [4, 12, 18, 6, 14],  // Right-back face
        [5, 9, 11, 7, 19],   // Left-back face
        [4, 18, 19, 5, 14],  // Top-right face
        [6, 15, 7, 19, 18],  // Bottom-back face
    ];

    // Generate the list of unique edges once at startup
    // This is more efficient than checking for duplicates every frame
    let edges = get_unique_edges(&faces);

    // Main rendering loop
    loop {
        // Clear the screen with black background
        clear_background(BLACK);

        // Update rotation angles
        // Different speeds for each axis create more interesting motion
        angle_x += 0.01;   // Rotate around X axis
        angle_y += 0.015;  // Rotate around Y axis (slightly faster)
        angle_z += 0.005;  // Rotate around Z axis (slowest)

        // Project all vertices to 2D screen coordinates
        // This involves:
        // 1. Rotating each vertex around all three axes
        // 2. Projecting the 3D point to 2D screen coordinates
        let projected: Vec<_> = vertices
            .iter()
            .map(|v| v.rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z).project())
            .collect();

        // Draw all unique edges with color cycling
        for (i, &(a, b)) in edges.iter().enumerate() {
            // Get the 2D coordinates of the edge endpoints
            let p1 = projected[a];
            let p2 = projected[b];
            // Calculate color based on edge index and current hue
            let color = hsv_to_rgb((h + i as f32 / edges.len() as f32) % 1.0, 1.0, 1.0);
            // Draw the edge as a line
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, color);
        }

                // Update hue for color cycling
                h += 0.002;  // Small increment for smooth color transition
                if h > 1.0 {
                    h -= 1.0;  // Wrap around when we reach the end of the color spectrum
                }
        
                // Wait for the next frame
                next_frame().await;
            }
        }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        // 💣 CURRENT ISSUES WITH THE RENDERING:
        // while using an unique edge list there is still visual artifacts because:
        // 1. still using 2D after projection, without proper depth testing
        // 2. Lines can be drawn in the wrong order (back-to-front instead of front-to-back)
        // 3. Floating-point rounding errors during projection cause small gaps
        // 4. No occlusion handling means lines can appear to pass through each other

        // ⚔️  SOLUTIONS:

        // ✅ Option 1: Implement edge-based depth sorting (Painter's Algorithm)
        /*
        // First, we need to track the 3D positions of our edges
        let mut edge_data: Vec<(f32, (usize, usize))> = edges.iter().map(|&(a, b)| {
            // Get the 3D positions of both endpoints
            let p1_3d = vertices[a].rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
            let p2_3d = vertices[b].rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
            // Calculate average Z depth of the edge
            let z_avg = (p1_3d.z + p2_3d.z) / 2.0;
            (z_avg, (a, b))
        }).collect();

        // Sort edges by Z depth (back to front)
        edge_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Draw edges in sorted order
        for (i, &(_, (a, b))) in edge_data.iter().enumerate() {
            let p1 = projected[a];
            let p2 = projected[b];
            let color = hsv_to_rgb((h + i as f32 / edges.len() as f32) % 1.0, 1.0, 1.0);
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, color);
        }
        */

        // ✅ Option 2: Macroquad's built-in 3D rendering
        /*
        // First, set up a 3D camera
        set_camera(&Camera3D {
            position: Vec3::new(0.0, 0.0, 5.0),  // Camera position
            target: Vec3::new(0.0, 0.0, 0.0),    // Look at center
            up: Vec3::new(0.0, 1.0, 0.0),        // Up vector
            ..Default::default()
        });

        // Then draw edges in 3D space
        for (i, &(a, b)) in edges.iter().enumerate() {
            // Get 3D positions of vertices
            let p1_3d = vertices[a].rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
            let p2_3d = vertices[b].rotate_x(angle_x).rotate_y(angle_y).rotate_z(angle_z);
            
            // Convert to Vec3 for draw_line_3d
            let v1 = Vec3::new(p1_3d.x, p1_3d.y, p1_3d.z);
            let v2 = Vec3::new(p2_3d.x, p2_3d.y, p2_3d.z);
            
            let color = hsv_to_rgb((h + i as f32 / edges.len() as f32) % 1.0, 1.0, 1.0);
            draw_line_3d(v1, v2, color);
        }
        */

        // Note: To use Option 2, :
        // 1. Add `use macroquad::experimental::camera::{Camera3D, set_camera};` at the top
        // 2. Add `use macroquad::experimental::collections::storage;` for 3D rendering
        // 3. Initialize 3D rendering with `storage::store(storage::Storage::new());`

//////////////////////////////////////////
// 3D Dodecahedron in Rust (Macroquad)////
//////////////////////////////////////////

// use macroquad::prelude::*;
// use std::collections::HashSet;

// // HSV to RGB conversion — rainbow wireframe
// fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
//     let i = (h * 6.0).floor();
//     let f = h * 6.0 - i;
//     let p = v * (1.0 - s);
//     let q = v * (1.0 - f * s);
//     let t = v * (1.0 - (1.0 - f) * s);

//     let (r, g, b) = match i as i32 % 6 {
//         0 => (v, t, p),
//         1 => (q, v, p),
//         2 => (p, v, t),
//         3 => (p, q, v),
//         4 => (t, p, v),
//         5 => (v, p, q),
//         _ => (0.0, 0.0, 0.0),
//     };

//     Color::new(r, g, b, 1.0)
// }

// // 3D point structure
// #[derive(Copy, Clone)]
// struct Point3D {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl Point3D {
//     fn new(x: f32, y: f32, z: f32) -> Self {
//         Self { x, y, z }
//     }

//     fn rotate(&self, angle_x: f32, angle_y: f32, angle_z: f32) -> Self {
//         let rx = self.rotate_x(angle_x);
//         let ry = rx.rotate_y(angle_y);
//         ry.rotate_z(angle_z)
//     }

//     fn rotate_x(&self, angle: f32) -> Self {
//         let cos = angle.cos();
//         let sin = angle.sin();
//         Self {
//             x: self.x,
//             y: self.y * cos - self.z * sin,
//             z: self.y * sin + self.z * cos,
//         }
//     }

//     fn rotate_y(&self, angle: f32) -> Self {
//         let cos = angle.cos();
//         let sin = angle.sin();
//         Self {
//             x: self.x * cos + self.z * sin,
//             y: self.y,
//             z: -self.x * sin + self.z * cos,
//         }
//     }

//     fn rotate_z(&self, angle: f32) -> Self {
//         let cos = angle.cos();
//         let sin = angle.sin();
//         Self {
//             x: self.x * cos - self.y * sin,
//             y: self.x * sin + self.y * cos,
//             z: self.z,
//         }
//     }

//     fn to_vec3(&self) -> Vec3 {
//         Vec3::new(self.x, self.y, self.z)
//     }
// }

// // Extract unique edges from face definitions
// fn get_unique_edges(faces: &[[usize; 5]]) -> Vec<(usize, usize)> {
//     let mut edge_set = HashSet::new();
//     let mut edges = Vec::new();

//     for face in faces {
//         for i in 0..5 {
//             let a = face[i];
//             let b = face[(i + 1) % 5];
//             let edge = if a < b { (a, b) } else { (b, a) };

//             if edge_set.insert(edge) {
//                 edges.push(edge);
//             }
//         }
//     }

//     edges
// }

// #[macroquad::main("True 3D Wireframe")]
// async fn main() {
//     let mut h = 0.0;
//     let mut angle_x = 0.0;
//     let mut angle_y = 0.0;
//     let mut angle_z = 0.0;

//     let phi = (1.0 + 5.0f32.sqrt()) / 2.0;
//     let vertices = vec![
//         Point3D::new(1.0, 1.0, 1.0),
//         Point3D::new(1.0, 1.0, -1.0),
//         Point3D::new(1.0, -1.0, 1.0),
//         Point3D::new(1.0, -1.0, -1.0),
//         Point3D::new(-1.0, 1.0, 1.0),
//         Point3D::new(-1.0, 1.0, -1.0),
//         Point3D::new(-1.0, -1.0, 1.0),
//         Point3D::new(-1.0, -1.0, -1.0),
//         Point3D::new(0.0, 1.0 / phi, phi),
//         Point3D::new(0.0, 1.0 / phi, -phi),
//         Point3D::new(0.0, -1.0 / phi, phi),
//         Point3D::new(0.0, -1.0 / phi, -phi),
//         Point3D::new(1.0 / phi, phi, 0.0),
//         Point3D::new(1.0 / phi, -phi, 0.0),
//         Point3D::new(-1.0 / phi, phi, 0.0),
//         Point3D::new(-1.0 / phi, -phi, 0.0),
//         Point3D::new(phi, 0.0, 1.0 / phi),
//         Point3D::new(phi, 0.0, -1.0 / phi),
//         Point3D::new(-phi, 0.0, 1.0 / phi),
//         Point3D::new(-phi, 0.0, -1.0 / phi),
//     ];

//     let faces = [
//         [0, 8, 10, 2, 16],
//         [0, 16, 17, 1, 8],
//         [0, 12, 4, 14, 8],
//         [8, 14, 5, 9, 1],
//         [16, 17, 3, 13, 2],
//         [1, 9, 11, 3, 17],
//         [2, 10, 6, 15, 13],
//         [3, 11, 7, 15, 13],
//         [4, 12, 18, 6, 14],
//         [5, 9, 11, 7, 19],
//         [4, 18, 19, 5, 14],
//         [6, 15, 7, 19, 18],
//     ];

//     let edges = get_unique_edges(&faces);

//     loop {
//         clear_background(BLACK);

//         // Setup real 3D camera
//         set_camera(&Camera3D {
//             position: vec3(0.0, 0.0, 6.0),
//             target: vec3(0.0, 0.0, 0.0),
//             up: vec3(0.0, 1.0, 0.0),
//             fovy: 45.0,
//             ..Default::default()
//         });

//         let rotated_vertices: Vec<Vec3> = vertices
//             .iter()
//             .map(|v| v.rotate(angle_x, angle_y, angle_z).to_vec3())
//             .collect();

//         for (i, &(a, b)) in edges.iter().enumerate() {
//             let p1 = rotated_vertices[a];
//             let p2 = rotated_vertices[b];
//             let color = hsv_to_rgb((h + i as f32 / edges.len() as f32) % 1.0, 1.0, 1.0);
//             draw_line_3d(p1, p2, color);
//         }

//         set_default_camera(); // Reset camera so UI can work if needed

//         angle_x += 0.01;
//         angle_y += 0.015;
//         angle_z += 0.005;

//         h = (h + 0.002) % 1.0;

//         next_frame().await;
//     }
// }


