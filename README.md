# LemoGUI

## Features
### Implement a GUI library dependent on wgpu
#### Plan
   1. Provide basic 2D graphics drawing API.
   2. Define basic widgets.
   3. Provide renderer to widgets through the drawing API.
   4. Provide event listeners to widgets.

### the third party library dependencies

- use wgpu-rs as the graphics backend
- use winit as window and event backend (async event is supported by futures)
- use ab_glyph to provide font shape parsing
- use image to provide the implementations of image encoding and decoding
