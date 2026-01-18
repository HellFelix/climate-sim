# Climate Simulator

A real-time climate simulation written in **Rust** using the **Bevy** engine.  
Models global temperature dynamics on a spherical planet surface, including day/night cycles, seasons, and heat diffusion.

📄 **Full technical report:** [report.pdf](report.pdf)

## Features

- Real-time visualization and simulation
- Orbital mechanics / solar incidence (seasons + day/night)
- Energy balance model with heat diffusion across the planet surface

## Tech Stack

- Rust
- Bevy (ECS + rendering)

## Running the project

### Requirements / Troubleshooting

Bevy uses GPU rendering. On some Linux systems you may need to install Vulkan drivers for your GPU (or the appropriate Mesa Vulkan packages) for Bevy to run correctly.
See the official Bevy setup/docs here: https://bevyengine.org/learn/book/getting-started/setup/

```bash
cargo run --release
```
