use std::fmt::Display;

use image::open;
use ocl::{Buffer, Device, Platform, ProQue};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn invert(&self) -> Color {
        Color {
            r: 255.0 - self.r,
            g: 255.0 - self.g,
            b: 255.0 - self.b,
        }
    }

    pub fn with_alpha(&self, alpha: f32) -> ColorA {
        ColorA {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    pub fn luminance(&self) -> f32 {
        const GAMMA: f32 = 2.4;

        const RED: f32 = 0.2126;
        const GREEN: f32 = 0.7152;
        const BLUE: f32 = 0.0722;

        fn transform(v: f32) -> f32 {
            let v = v / 255.0;
            if v <= 0.03928 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(GAMMA)
            }
        }

        let r = transform(self.r);
        let g = transform(self.g);
        let b = transform(self.b);

        r * RED + g * GREEN + b * BLUE
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct ColorA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Display for ColorA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

pub fn extract_pallete(
    image_path: &str,
    num_centroids: u32,
    max_iterations: u32,
) -> Result<Vec<Color>, ocl::Error> {
    // Load image
    let img = open(image_path).expect("Failed to open image").to_rgba8();
    let (width, height) = img.dimensions();
    let num_pixels = (width * height) as usize;

    // Convert image to flat array of RGBA pixels
    let pixels: Vec<u8> = img.as_raw().clone();

    // Initialize centroids randomly (in range 0-255 for RGB channels)
    let mut rng = rand::thread_rng();
    let mut centroids: Vec<ocl::prm::Float3> = (0..num_centroids)
        .map(|_| {
            ocl::prm::Float3::new(
                rng.gen::<f32>() * 255.0,
                rng.gen::<f32>() * 255.0,
                rng.gen::<f32>() * 255.0,
            )
        })
        .collect();

    // OpenCL program
    let src = include_str!("../assets/kmeans.cl");
    let platform = Platform::list()
        .into_iter()
        .find(|p| p.name().unwrap_or_default().contains("NVIDIA"))
        .expect("NVIDIA platform not found!");

    let device = Device::first(platform).expect("No devices found on platform!");

    // Create OpenCL context, program, and buffers
    let pro_que = ProQue::builder()
        .src(src)
        .device(device)
        .dims(num_pixels)
        .build()?;

    let image_buffer = Buffer::<u8>::builder()
        .queue(pro_que.queue().clone())
        .flags(ocl::flags::MEM_READ_ONLY)
        .len(pixels.len())
        .copy_host_slice(&pixels)
        .build()?;

    let centroid_buffer = Buffer::<ocl::prm::Float3>::builder()
        .queue(pro_que.queue().clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(num_centroids)
        .copy_host_slice(&centroids)
        .build()?;

    let cluster_buffer = Buffer::<u32>::builder()
        .queue(pro_que.queue().clone())
        .flags(ocl::flags::MEM_WRITE_ONLY)
        .len(num_pixels)
        .build()?;

    let kernel = pro_que
        .kernel_builder("kmeans_cluster")
        .arg(&image_buffer)
        .arg(&centroid_buffer)
        .arg(&cluster_buffer)
        .arg(num_centroids)
        .arg(num_pixels as u32)
        .build()?;

    // Iterate to refine centroids
    for _iteration in 0..max_iterations {
        // Run kernel
        unsafe {
            kernel.enq()?;
        }

        pro_que.finish()?;

        // Read cluster assignments back to host
        let mut cluster_assignments = vec![0u32; num_pixels];
        cluster_buffer.read(&mut cluster_assignments).enq()?;

        // Recompute centroids
        let mut new_centroids = vec![ocl::prm::Float3::new(0.0, 0.0, 0.0); num_centroids as usize];
        let mut counts = vec![0u32; num_centroids as usize];

        for (i, &cluster) in cluster_assignments.iter().enumerate() {
            let r = pixels[i * 4] as f32;
            let g = pixels[i * 4 + 1] as f32;
            let b = pixels[i * 4 + 2] as f32;

            new_centroids[cluster as usize][0] += r;
            new_centroids[cluster as usize][1] += g;
            new_centroids[cluster as usize][2] += b;
            counts[cluster as usize] += 1;
        }

        for c in 0..num_centroids as usize {
            if counts[c] > 0 {
                new_centroids[c][0] /= counts[c] as f32;
                new_centroids[c][1] /= counts[c] as f32;
                new_centroids[c][2] /= counts[c] as f32;
            }
        }

        // Write new centroids to the buffer
        centroid_buffer.write(&new_centroids).enq()?; // Ensure the new centroids are sent to the GPU

        centroids = new_centroids;
    }

    let final_centroids: Vec<Color> = centroids
        .iter()
        .map(|c| Color {
            r: c[0],
            g: c[1],
            b: c[2],
        })
        .collect();

    Ok(final_centroids)
}
