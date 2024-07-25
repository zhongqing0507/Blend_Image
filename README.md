# BlendImage

BlendImage CLI 是一款基于命令行的图像处理工具，专为快速、便捷地混合及调整图像而设计。当前版本主要功能包括混合两幅图像。


## 特性

- **图像混合**: 能够轻松混合两张图片，并支持设定亮度、饱和度、对比度、伽马变化和选择不同的混合模式。
- **广泛兼容**: 支持JPEG、PNG、BMP等多种常见图像格式，确保通用性。
- **混合模式多样性**: 包括但不限于叠加(Over)、正片叠底(Multiply)、屏幕(Screen)等模式，满足不同视觉效果需求。
- **命令行便捷**: 无需图形界面，通过简单的命令行指令即可完成图像处理任务。


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
- addition
- exclusion
- subtract
- screen

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
# 将图片1和图片2以 multiply 模式混合
./target/release/image_blend  ./data/src1.png  ./data/src2.png -o ./data/blend/ -m multiply
```

```sh
# 将图片1和图片2以 multiply 模式混合,并调整亮度对比对饱和度和伽马变换
./target/release/image_blend  ./data/src1.png  ./data/src2.png -o ./data/blend/ -m multiply  
--brightness=0  --contrast=0.0 --gamma=1  --saturation=50
```

```sh
# 将图片1和图片2以 multiply 模式混合,并调整亮度对比对饱和度和伽马变换，以及重新对颜色进行调整
./target/release/image_blend  ./data/src1.png  ./data/src2.png -o ./data/blend/ -m multiply  
--brightness=0  --contrast=0.0 --gamma=1  --saturation=50  --colorize  --colorize-color=255,128,128

```
