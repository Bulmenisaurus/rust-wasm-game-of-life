mod utils;

use wasm_bindgen::prelude::*;

//* https://rustwasm.github.io/docs/book/
//* https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: u8,
}

impl RGBA {
    fn into_array(&self) -> [u8; 4] {
        [self.red, self.blue, self.green, self.alpha]
    }
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(&self) -> Complex {
        self.multiply_by_complex(*self)
    }

    fn multiply_by_complex(&self, by: Complex) -> Complex {
        Complex {
            real: (self.real * by.real - self.imaginary * by.imaginary),
            imaginary: (self.real * by.imaginary + self.imaginary * by.real),
        }
    }

    fn add_to_complex(&self, to: Complex) -> Complex {
        Complex {
            real: self.real + to.real,
            imaginary: self.imaginary + to.imaginary,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex {
            real: 0.,
            imaginary: 0.,
        }
    }
}
fn seems_to_converge(c: Complex) -> bool {
    c.magnitude() < 10.
}

fn strange_converge(c: Complex) -> bool {
    c.real.abs() < 10. || c.imaginary.abs() < 10.
}

fn get_pixel_color(x: i32, y: i32, width: i32, height: i32, scale_factor: f64) -> RGBA {
    let (mut a, mut c, mut e, mut f, mut g) = (
        Complex::default(),
        Complex::default(),
        Complex::default(),
        Complex::default(),
        Complex::default(),
    );

    let mut z = Complex {
        real: f64::from(x - width / 2) * scale_factor,
        imaginary: f64::from(y - height / 2) * scale_factor,
    };

    // book suggests 0.35 + 0.35i
    let u = Complex {
        real: 0.35,
        imaginary: 0.35,
    };

    let amount_iterations = 50;
    let mut result_i = amount_iterations;
    for i in 0..amount_iterations {
        a = z;

        e = a.square().add_to_complex(u);
        c = e.square().add_to_complex(u);
        f = c.square().add_to_complex(u);
        a = f.square().add_to_complex(u);

        z = a;

        if !seems_to_converge(z) {
            result_i = i;
            break;
        }
    }
    // convert integer in range [0, amount_iterations) to [0, 256)
    let color = 255. / f64::from(amount_iterations) * f64::from(result_i);

    let color: u8 = color as u8;
    if strange_converge(c) {
        RGBA {
            red: color,
            blue: color,
            green: color,
            alpha: 255,
        }
    } else {
        RGBA {
            red: 255,
            blue: 255,
            green: 255,
            alpha: 255,
        }
    }
}

#[wasm_bindgen]
pub fn render(width: i32, height: i32, zoom: i32) -> *const u8 {
    let mut data = Vec::new();
    let scale_factor: f64 = 1. / f64::from(width * zoom);

    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel_color(x, y, width, height, scale_factor);

            data.extend(pixel.into_array())
        }
    }

    return data.as_ptr();
}
