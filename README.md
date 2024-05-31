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
![image](https://github.com/atomicbeef/software-renderer/assets/10298038/027fdca8-e1d4-4345-b784-d2c3817a12eb)
![image](https://github.com/atomicbeef/software-renderer/assets/10298038/72a3210e-9790-4f28-b775-b5322b47f690)
![image](https://github.com/atomicbeef/software-renderer/assets/10298038/3840b2d6-9ee2-4522-8d5c-4eb695755295)

# Limitations
Some very small cracks can be seen occasionally between triangles with shared edges. I'm not sure what the exact cause of this is, but it's probably because of some imprecision introduced during rasterization. [This Reddit thread](https://www.reddit.com/r/GraphicsProgramming/comments/1cz6tqo/holes_in_shared_edges_of_triangles_in_software/) might be useful to anyone suffering from the same problems, but I ultimately couldn't figure out how to fix this myself. If you think you know how it could be fixed, I'd love to hear your thoughts!