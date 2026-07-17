#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate panic_halt;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

const GRID_SIZE: usize = 64;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut psi = [0.0f32; GRID_SIZE * GRID_SIZE];
    let mut pi = [0.0f32; GRID_SIZE * GRID_SIZE];

    // libm-free algebraic seeding (deterministic chaotic initial state)
    for i in 0..GRID_SIZE*GRID_SIZE {
        psi[i] = (((i * 137) % GRID_SIZE) as f32 / GRID_SIZE as f32) - 0.5;
    }

    // Symplectic Wave Dispatch Loop
    for _cycle in 0.. {
        let mut energy = 0.0f32;

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let idx = y * GRID_SIZE + x;
                let up = psi[((y + GRID_SIZE - 1) % GRID_SIZE) * GRID_SIZE + x];
                let down = psi[((y + 1) % GRID_SIZE) * GRID_SIZE + x];
                let left = psi[y * GRID_SIZE + ((x + GRID_SIZE - 1) % GRID_SIZE)];
                let right = psi[y * GRID_SIZE + ((x + 1) % GRID_SIZE)];
                let center = psi[idx];

                let laplacian = up + down + left + right - 4.0 * center;
                
                let mag = center.abs();
                let mag2 = mag * mag;
                let nonlinear = 1.618f32 * (mag2 * mag2) - mag2;
                
                let force = laplacian + nonlinear;

                pi[idx] += force * 0.05;
                pi[idx] *= 0.992;
                psi[idx] += pi[idx] * 0.05;

                energy += psi[idx].abs();
            }
        }

        let harmony = 1.0f32 / (1.0f32 + (energy / (GRID_SIZE*GRID_SIZE) as f32 - 1.0).abs());
        if harmony > 0.98 {
            unsafe { core::arch::asm!("ebreak"); }
        }
    }
}
