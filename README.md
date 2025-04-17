# ColorCoded Dodecahedron

<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&size=22&duration=3000&pause=1000&color=F75C7E&center=true&vCenter=true&multiline=true&width=800&lines=A+3D+wireframe+dodecahedron+visualization+built+with+Rust+and+Macroquad.;This+project+demonstrates+3D+geometry%2C+color+theory%2C+and+real-time+graphics+programming." alt="Typing SVG" />
</p>

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Macroquad](https://img.shields.io/badge/Macroquad-FF6B6B?style=for-the-badge&logo=opengl&logoColor=white)
![3D Graphics](https://img.shields.io/badge/3D_Graphics-00BFFF?style=for-the-badge&logo=opengl&logoColor=white)
![Open Source](https://img.shields.io/badge/Open_Source-00FF00?style=for-the-badge&logo=github&logoColor=white)
![Learning Project](https://img.shields.io/badge/Learning_Project-FFD700?style=for-the-badge&logo=book&logoColor=black)

</div>

## 📚 About This Project

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

This project was created as a learning exercise to explore several key concepts in computer graphics and programming:

- 🎓 **Educational Focus**: The code is heavily commented to explain each component and decision
- 🔍 **Transparent Development**: Includes commented-out solutions to demonstrate different approaches
- 📝 **Documentation**: Detailed explanations of both working and alternative implementations
- 🧠 **Learning Journey**: Shows the evolution of solutions and why certain approaches were chosen
- 💡 **Problem-Solving**: Demonstrates how to tackle common 3D graphics challenges

The project serves as both a functional visualization and an educational resource, making it ideal for:
- Learning 3D graphics programming
- Understanding color theory in practice
- Exploring Rust's capabilities
- Studying real-time rendering techniques

</div>

## 🎨 Features

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- 🌟 Real-time 3D wireframe dodecahedron rendering
- 🎨 Smooth color cycling using HSV color space
- ⚡ Efficient edge rendering with unique edge list
- 🔍 Perspective projection for 3D depth
- 🔄 Continuous rotation animation

</div>

## 🛠️ Technical Details

### 3D Geometry
<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- 📐 Uses a regular dodecahedron (12 pentagonal faces, 20 vertices)
- 🌟 Vertices positioned using the golden ratio (φ) for perfect regularity
- 🔄 Implements 3D rotation around all axes (X, Y, Z)
- 🔍 Perspective projection for depth perception

</div>

### Color System
<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- 🎨 HSV (Hue, Saturation, Value) color model
- 🌈 Smooth color transitions across the spectrum
- 🎯 Each edge cycles through colors independently
- 🌈 Full color spectrum coverage (0° to 360°)

</div>

### Rendering
<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- ✏️ Wireframe rendering with clean lines
- 🔄 Unique edge list to prevent duplicate drawing
- 📐 2D projection with perspective
- ⚡ Efficient vertex transformation

</div>

## ⚠️ Known Issues

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

The current implementation has some visual artifacts due to:
1. 🎯 2D rendering after projection (no proper depth testing)
2. 🔄 Potential line drawing order issues
3. 🔢 Floating-point rounding errors
4. 👁️ No occlusion handling

Potential solutions (commented in code):
1. 🎨 Edge-based depth sorting (Painter's Algorithm)
2. 🖥️ Using Macroquad's built-in 3D rendering

</div>

## 📋 Requirements

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- 🦀 Rust (latest stable version)
- 📦 Cargo (Rust's package manager)
- 🎮 Macroquad graphics library

</div>

## 🚀 Installation

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ColorCoded.git
cd ColorCoded
```

2. Build and run:
```bash
cargo run
```

</div>

## 📁 Project Structure

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

```
ColorCoded/
├── src/
│   └── main.rs      # Main application code
├── Cargo.toml       # Project configuration and dependencies
└── README.md        # This file
```

</div>

## 💻 Code Overview

### Key Components

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

1. **Point3D Structure**
   - 📐 Represents 3D vertices
   - 🔄 Implements rotation and projection
   - 🎯 Handles 3D transformations

2. **Color System**
   - 🎨 HSV to RGB conversion
   - 🌈 Smooth color cycling
   - 🎯 Edge-based color assignment

3. **Geometry**
   - 📐 Dodecahedron vertex definitions
   - 🎯 Face definitions
   - 🔄 Edge generation and management

4. **Rendering**
   - 🖥️ Projection to 2D screen space
   - ✏️ Edge drawing
   - 🔄 Animation loop

</div>

### Main Functions

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- `hsv_to_rgb()`: Converts HSV colors to RGB
- `get_unique_edges()`: Generates unique edge list
- `Point3D::project()`: Handles 3D to 2D projection
- `Point3D::rotate_*()`: Implements 3D rotation

</div>

## 🔮 Future Improvements

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

1. 🎯 Implement proper depth sorting
2. 🎮 Add interactive controls
3. 📐 Support for different polyhedrons
4. ⚡ Performance optimizations
5. 🎨 Add shader effects

</div>

## 🤝 Contributing

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

Feel free to submit issues and enhancement requests!

</div>

## 📄 License

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

This project is licensed under the MIT License - see the LICENSE file for details.

</div>

## 🙏 Acknowledgments

<div style="background-color: #1E1E1E; padding: 20px; border-radius: 10px;">

- 🎮 Macroquad for the graphics framework
- 🦀 The Rust community for excellent tools and documentation
- 📐 Platonic solids for the beautiful geometry

</div>

---

<div align="center">

[![GitHub stars](https://img.shields.io/github/stars/yourusername/ColorCoded?style=social)](https://github.com/yourusername/ColorCoded)
[![GitHub forks](https://img.shields.io/github/forks/yourusername/ColorCoded?style=social)](https://github.com/yourusername/ColorCoded)
[![GitHub watchers](https://img.shields.io/github/watchers/yourusername/ColorCoded?style=social)](https://github.com/yourusername/ColorCoded)

</div> 