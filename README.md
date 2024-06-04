# software-renderer
A 3D renderer written in Rust. Entirely software based, so no graphics card needed! This renderer was created as part of [this excellent course](https://pikuma.com/courses/learn-3d-computer-graphics-programming) by Gustavo Pezzi.

# Features
- OBJ file rendering, with PNG texture support
- Per face lighting
- Backface culling
- FPS style camera control
- Wireframe mode
- Controllable model translation, rotation, and scaling
- Projective space clipping
- Rendering of scenes defined in JSON
- Subpixel rasterization

# Usage
`cargo run --release <path to mesh.obj or scene.json>`

| Button       | Function                  |
| -----------  | -----------               |
| W            | Move camera forward       |
| A            | Move camera left          |
| S            | Move camera backwards     |
| D            | Move camera right         |
| Space        | Move camera up            |
| Left control | Move camera down          |
| Left arrow   | Rotate camera left        |
| Right arrow  | Rotate camera right       |
| Up arrow     | Rotate camera up          |
| Down arrow   | Rotate camera down        |
| Left shift   | Move camera faster        |
| 1            | Vertex rendering with wireframe     |
| 2            | Wireframe rendering        |
| 3            | Filled face rendering     |
| 4            | Filled face rendering with wireframe         |
| 5            | Textured rendering     |
| 6            | Textured rendering with wireframe |
| 7            | Depth buffer view                 |
| C            | Toggle backface culling         |
| L            | Enable shading     |
| U            | Disable shading |
| T            | Toggle model translation     |
| R            | Toggle model rotation |
| G            | Toggle model scaling |
| X            | Toggle model rotation around the x-axis     |
| Y            | Toggle model rotation around the y-axis |
| Z            | Toggle model rotation around the z-axis |
| P            | Reset model rotation |
| F            | Flip texture UVs (useful if the model's texture is upside down) |

# Screenshots
<img width="595" alt="Jets Wireframe" src="https://github.com/atomicbeef/software-renderer/assets/10298038/62891cae-5ee6-4123-a5bd-1113a511e1e9">
<img width="718" alt="Jets Unlit" src="https://github.com/atomicbeef/software-renderer/assets/10298038/cb320b58-8b2a-4e78-8b4e-89f1600cc71b">
<img width="441" alt="Sphere" src="https://github.com/atomicbeef/software-renderer/assets/10298038/e6e4ec0e-5205-4a8b-a11d-8b9e0f5859d0">
<img width="488" alt="Crab Lit" src="https://github.com/atomicbeef/software-renderer/assets/10298038/04240a35-7086-4802-8cdf-f13bc30bc741">
<img width="509" alt="Crab Depth" src="https://github.com/atomicbeef/software-renderer/assets/10298038/092178a0-01a1-407f-af99-a44d71dbce4a">
