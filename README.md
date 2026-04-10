# ADD REFERENCES BEFORE SUBMITTING!!!!!!!!!!!!!!
# ALSO ADD FILE TREE DIAGRAMS IN INSTRUCTIONS.MD

# Image Classification in Rust

## 1. Project Title & Authors

**Image Classification in Rust** by Sean McMurray and Ryan Buck.

## 2. Project Description

This project uses the k-Nearest Neighbors algorithm to classify images from the MNIST Handwritten Digits (28x28 grayscale) dataset.
The project runs in sequence and in parallel and measures the execution time, speedup, efficiency, classification accuracy, and throughput for each mode.
The project has two modes for running in parallel: using Rayon and using built-in thread pools. Each mode will be tested.
The project will also repeat on a second device with different hardware, measuring the same metrics.

## 3. Prerequisites

### Rust

Please ensure the Rust compiler (rustc) is updated to version 1.93.1 or later.
Trying to compile the project with an earlier version may result in errors.

### System Requirements

This project will likely run well on any modern device. At a minimum, you should have:

| Component | Requirement |
|-----------|-------------|
| **OS** | Windows 10, macOS 10.15, or Linux (kernel 4.15) |
| **CPU** | x86-64, 2 cores |
| **RAM** | 512 MB |
| **Disk** | 400 MB |

Anything newer/better than these specifications is likely to run the project with no issues.

### Dependencies

You must download the dataset and place it in the correct folder. Instructions for doing so are listed [here](./data/INSTRUCTIONS.md).

The dependencies in [`Cargo.toml`](./Cargo.toml) should be automatically installed upon building the project.

## 4. Setup Instructions

1. Extract the .zip file.

2. Follow the instructions in [data/INSTRUCTIONS.md](./data/INSTRUCTIONS.md) to download the dataset.

3. Build and run the project from the command line with `cargo run`.

## 5. References

No references.
