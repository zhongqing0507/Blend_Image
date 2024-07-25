extern crate image;
extern crate imageproc;

use image::{GenericImage, ImageBuffer, Rgba};
// use imageproc::pixelops::linear;
use std::path::Path;

#[test]
fn test_main() {
    // 读取两幅图像
    let img1 = image::open(Path::new("data/beijing-dem-rgb.tif")).unwrap().to_rgba8();
    let img2 = image::open(Path::new("data/beijing-hillshade-rgb.tif")).unwrap().to_rgba8();

    // 应用Color Burn混合模式
    let combined_img = combine_images(&img1, &img2);

    // 保存结果
    combined_img.save("dodge_image.png").unwrap();
}


// 快速除以255的近似函数
// fn qt_div_255(x: u8) -> u8 {
//     ((x as u16 + (x as u16 >> 8) + 0x80) >> 8) as u8
// }
// fn qt_div_255(x: i64) -> u8 {
//     ((x + (x >> 8) + 0x80) >> 8) as u8
// }

// 可以用
// fn color_burn_op(dst: u8, src: u8, da: u8, sa: u8) -> u8 {
//     let src_da = src as u32 * da as u32;
//     let dst_sa = dst as u32 * sa as u32;
//     let sa_da = sa as u32 * da as u32;

//     let temp = src as u32 * (255 - da as u32) + dst as u32 * (255 - sa as u32);

//     if src_da + dst_sa < sa_da {
//         qt_div_255(temp)
//     } else if src == 0 {
//         qt_div_255(dst_sa + temp)
//     } else {
//         qt_div_255(sa as u32 * (src_da + dst_sa - sa_da) / src as u32 + temp)
//     }
// }

fn qt_div_255(x: u64) -> u8 {
    ((x + (x >> 8) + 0x80) >> 8) as u8
}

fn color_dodge_op(dst: u8, src: u8, da: u8, sa: u8) -> u8 {
    let sa_da = sa as u64 * da as u64;
    let dst_sa = dst as u64 * sa as u64;
    let src_da = src as u64 * da as u64;

    let temp = src as u64 * (255 - da as u64) + dst as u64 * (255 - sa as u64);

    let result = if src_da + dst_sa > sa_da {
        qt_div_255(sa_da + temp)
    } else if src == sa || sa == 0 {
        qt_div_255(temp)
    } else {
        let denominator = 255 - 255 * src as u64 / sa as u64;
        if denominator == 0 {
            255 // Avoid division by zero
        } else {
            qt_div_255(255 * dst_sa / denominator + temp)
        }
    };

    result
}


// 可以用
fn softlight_op(dst: u8, src: u8, da: u8, sa: u8) -> u8 {
    let dst = dst as i64;
    let src = src as i64;
    let da = da as i64;
    let sa = sa as i64;

    let src2: i64 = src << 1;
    let dst_np = if da != 0 { (255 * dst) / da } else { 0 };
    let temp = (src * (255 - da) + dst * (255 - sa)) * 255;

    let result = if src2 < sa {
        (dst * (sa * 255 + (src2 - sa) * (255 - dst_np)) + temp) / 65025
    } else if 4 * dst <= da {
        (dst * sa * 255 + da * (src2 - sa) *
          (((16 * dst_np - 12 * 255) * dst_np + 3 * 65025) * dst_np / 65025) + temp) / 65025
    } else {
        (dst * sa * 255 + da * (src2 - sa) *
          ((f64::sqrt(dst_np as f64 * 255.0) as i64) - dst_np) + temp) / 65025
    };

    result as u8
}
// Color Burn 混合模式操作函数
// fn color_burn_op(dst: u8, src: u8, da: u8, sa: u8) -> u8 {
//     // 注意：此处直接使用了u8的运算，未进行浮点运算，因此与之前JavaScript的实现细节有差异
//     // 根据具体需求，可能需要调整为更精确的浮点运算实现
//     let temp = src.saturating_sub(255 - da) * dst / 255;
//     if temp > 0 { temp } else { 0 }
// }

fn combine_images(img1: &ImageBuffer<Rgba<u8>, Vec<u8>>, img2: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img1.dimensions();
    assert_eq!(img2.dimensions(), (width, height));

    let mut combined_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);

            // let r = softlight_op(pixel1[0] as u32, pixel2[0] as u32, pixel1[3] as u32, pixel2[3] as u32) as u8;
            // let g =  softlight_op(pixel1[1] as u32, pixel2[1] as u32, pixel1[3] as u32, pixel2[3] as u32) as u8;
            // let b = softlight_op(pixel1[2] as u32, pixel2[2] as u32, pixel1[3] as u32, pixel2[3] as u32) as u8;

            // let r = softlight_op(pixel1[0] , pixel2[0] , pixel1[3] , pixel2[3] ) as u8;
            // let g = softlight_op(pixel1[1] , pixel2[1], pixel1[3] , pixel2[3] ) as u8;
            // let b = softlight_op(pixel1[2] , pixel2[2] , pixel1[3] , pixel2[3] ) as u8;

            
            let r = color_dodge_op(pixel1[0] , pixel2[0] , pixel1[3] , pixel2[3] ) as u8;
            let g = color_dodge_op(pixel1[1] , pixel2[1], pixel1[3] , pixel2[3] ) as u8;
            let b = color_dodge_op(pixel1[2] , pixel2[2] , pixel1[3] , pixel2[3] ) as u8;

            let a = pixel2[3];

            *combined_img.get_pixel_mut(x, y) = Rgba([r, g, b, a]);
        }
    }

    combined_img
}




use ndarray::Array1;

#[test]
fn test_main2() {
    let src = Array1::from(vec![0.588, 0.392, 0.784]); // RGB 源像素值，归一化
    let dst = Array1::from(vec![0.196, 0.588, 0.392]); // RGB 目标像素值，归一化
    let sa = 0.5;  // 源透明度，归一化
    let da = 1.0;  // 目标透明度，归一化

    softlight_op_new(dst, src, da, sa);

    // println!("混合后的 RGB 像素值: {:?}", blended);
}
fn softlight_op_new(dst: Array1<f32>, src: Array1<f32>, da: f32, sa: f32){
    let src2 = &src * 2.0;
    let dst_np = if da != 0.0 {
        (&dst * 1.0) / da
    } else {
        Array1::zeros(dst.len())
    };
    let temp = (&src * (1.0 - da) + &dst * (1.0 - sa)) * 1.0;

    let result:Vec<_> = src2
        .iter()
        .zip(dst.iter())
        .zip(dst_np.iter())
        .map(|((&src2, &dst), &dst_np)| {
            if src2 < sa {
                (dst * (sa * 1.0 + (src2 - sa) * (1.0 - dst_np)) + temp.clone()) / 1.0
            } else if 4.0 * dst <= da {
                (dst * sa * 1.0 + da * (src2 - sa)
                    * (((16.0 * dst_np - 12.0 * 1.0) * dst_np + 3.0) * dst_np / 1.0) + temp.clone()) / 1.0
            } else {
                (dst * sa * 1.0 + da * (src2 - sa)
                    * ((dst_np.sqrt() * 1.0) - dst_np) + temp.clone()) / 1.0
            }
        })
        .collect();
    println!("{:?}", result);
    // Array1::from(result)
}

