## 🚀 Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

To run **AdaOS**, you need to have the following installed:
* [Rust](https://www.rust-lang.org/tools/install) (Nightly toolchain needed)
* [QEMU](https://www.qemu.org/download/) (System emulators)

### Installation & Run

1. **Clone the repository**

   ```bash
   git clone https://github.com/yatsenk/AdaOS.git
   cd AdaOS
   
2. **Build and Run**

   ```bash
   cargo run

### Testing

1. **Normal testing**
   ```bash
   cargo test
   ```

2. **Library testing**
   ```bash
   cargo test --lib
   ```

3. **Integration testing**
   ```bash
   cargo test --test basic_boot
   cargo test --test stack_overflow
   ```
   
Note: If you encounter any issues with the target architecture, ensure you have the necessary rust-src component installed via rustup component add rust-src.
