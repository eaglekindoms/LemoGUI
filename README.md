# LemoGUI

### 目标
1. 实现一个图形用户界面库，提供一些基本组件
   * [x] Button 
   * [x] Text Input
   * [x] Image Display
2. 组件实现与底层渲染解耦，可绑定不同的图形驱动库
   * [x] wgpu 


### the third party library dependencies

- use wgpu-rs as the graphics backend
- use winit as window and event backend (async event is supported by futures)
- use ab_glyph to provide font shape parsing
- use image to provide the implementations of image encoding and decoding
