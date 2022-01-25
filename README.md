# LemoGUI

## Features

### Implement a GUI library depend on wgpu

#### Plan

1. Provide basic 2D graphics drawing API.
2. Define basic widgets.
3. Provide renderer to widgets through the drawing API.
4. Provide event listeners to widgets.

##### widgets

- [x] frame 窗体容器框
- [x] panel 控件容器面板
- [x] button 按钮
- [x] text input 文本输入框
- [ ] text input area 文本输入域
- [ ] image label 图像面板
- [ ] check box 单选框
- [ ] list 列表
- [ ] tree 树形组件
- [ ] menu 菜单
- [ ] menu item 菜单选项
- [ ] message box 消息框
- [ ] progress bar 进度条
- [ ] scroll bar 滚动条
- [ ] scroll panel 滚动面板
- [ ] scroll viewer 滚动视图
- [ ] grid 网格

### the third party library dependencies

- use wgpu-rs as the graphics backend
- use winit/sdl2 as window and event backend (async event is supported by futures)
- use ab_glyph to provide font shape parsing
- use image to provide the implementations of image encoding and decoding
