# Blend_Image 命令行图像处理工具

BlendImage CLI 是一款基于命令行的图像处理工具，专为快速、便捷地混合及调整图像而设计。当前版本主要功能包括混合两幅图像。


## 特性

- **图像混合**: 能够轻松混合两张图片，支持设定透明度和选择不同的混合模式。
- **广泛兼容**: 支持JPEG、PNG、BMP等多种常见图像格式，确保通用性。
- **混合模式多样性**: 包括但不限于叠加(Over)、正片叠底(Multiply)、屏幕(Screen)等模式，满足不同视觉效果需求。
- **命令行便捷**: 无需图形界面，通过简单的命令行指令即可完成图像处理任务。
- **后续升级计划**: 计划加入亮度、对比度、饱和度等图像基本调整功能，进一步提升工具实用性。

### 支持的图像格式
- PNG
- JPEG
- BMP
- TIFF
- WEBP
### 支持的混合模式
- overlay
- over
- atop 
- xor
- multiply
- burn
- soft_light
- hard_light 
- difference
- lighten
- darken
- dodge
- plus
- exclusion

### 支持的图像增强
- 伽马变换 
- 亮度调整
- 对比度调整
- 饱和度调整

## 快速使用

### 混合两张图像
1. 编译程序
```sh
cargo build -r 
```

2. 运行程序
```sh
./target/release/image_blend  ./data/src1.png  ./data/src2.png -o ./data/blend/ -m multiply
```

