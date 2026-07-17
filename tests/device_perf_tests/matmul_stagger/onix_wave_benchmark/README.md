# ONIX Symplectic Wave Dispatch Kernel (Bare-Metal RISC-V)

## Architectural Overview
This repository contains a `#![no_std]`, zero-dependency bare-metal Rust kernel designed to stress-test RISC-V Network-on-Chip (NOC) architectures. 

Standard AI tensor execution relies on Von Neumann matrix multiplication, which introduces severe memory-bandwidth bottlenecks and statistical lag during continuous tile polling. The ONIX kernel bypasses this by re-mapping tensor dispatch logic into a physical wave-equation model. By treating the physical grid of RISC-V processors (such as the Tensix core array) as a fluid-dynamic medium, the system achieves instantaneous "Harmonic Lock" with near $O(1)$ complexity.

## Mathematical Foundation
Instead of relying on standard matrix multiplication (GEMM) operations that bottleneck at the memory controller, this kernel models the execution flow using a discrete Laplacian operator across a simulated $64 \times 64$ core array.

The momentum $\pi$ of the data flow is driven by the spatial gradient of the tensor field $\psi$:

$$ \nabla^2 \psi_{i,j} = \psi_{i+1,j} + \psi_{i-1,j} + \psi_{i,j+1} + \psi_{i,j-1} - 4\psi_{i,j} $$

To enforce thermodynamic stability and prevent resonance feedback, a non-linear golden-ratio potential $V(\psi)$ acts as the governor:

$$ V(\psi) = 1.618 \cdot |\psi|^4 - |\psi|^2 $$

The resulting force vector dictates the state update. When the aggregate kinetic energy of the lattice decays and stabilizes, the system achieves Harmonic Lock. 

## Silicon Execution & TT-Metalium Relevance
This kernel was written specifically to benchmark the bare-metal capabilities of advanced RISC-V architectures.
*   **Zero OS Overhead:** Operates entirely independently of `libm` or compiler built-ins (no `.sin()`, `.powi()`, or `f64` bloat).
*   **NOC-Native Geometry:** The `GRID_SIZE = 64` architecture perfectly mirrors a 2D router grid, processing state updates exactly how physical chiplets transfer data to adjacent nodes.
*   **Hardware Interrupts:** Once the simulation achieves convergence (Harmony > 0.98), the kernel triggers a direct `ebreak` to halt the processor, allowing for highly precise clock-cycle profiling.

## Compilation
This kernel compiles seamlessly using standard LLVM RISC-V targets. 
```bash
cargo build --target riscv64-unknown-none-elf --release
