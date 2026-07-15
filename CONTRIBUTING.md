# Contributing to KnightOS

Thank you for your interest in KnightOS. This project is a personal learning journey focused on handcrafted operating system design. 

Because the primary goal of this project is self-education, the core architectural code is written by the main author. However, contributions in the form of detailed bug reports, documentation improvements, unit tests, and structural feedback are highly welcome. 

If you want to experiment with the codebase, you are encouraged to fork the repository and build your own custom version of the operating system.

---

## Setting Up Your Local Environment

To get a reproducible local development environment running on your machine, follow these steps.

### 1. Install System Tools
Ensure your system has the required build chain and emulators.

**Debian / Ubuntu:**
```bash
sudo apt update && sudo apt install -y build-essential make binutils grub-pc-bin xorriso mtools qemu-system-x86
```

**Arch Linux:**
```bash
sudo pacman -Syu && sudo pacman -S --needed base-devel make grub libisoburn mtools qemu-desktop
```

### 2. Configure Rust Nightly
KnightOS relies on specific bare-metal features from the Rust Nightly toolchain.

```bash
# Install rustup if it is not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure the toolchain and target requirements
rustup toolchain install nightly
rustup component add llvm-tools-preview rust-src --toolchain nightly
cargo install cargo-binutils
```

### 3. Build and Run
Clone your fork and build the bootable ISO:
```bash
git clone https://github.com/your-username/knightos.git
cd knightos
make build
make run
```

---

## Code Style and Formatting

To keep the codebase readable and consistent, standard Rust formatting rules are enforced.

*   **Formatting:** All Rust code must be formatted using `rustfmt` before committing.
*   **Lints:** We use `clippy` to catch common mistakes and enforce idiomatic Rust.
*   **Bare-metal Constraints:** Since this is a `no_std` bare-metal environment, avoid using heap allocations inside low-level interrupt handlers.

Run the following commands to check your code style locally:

```bash
# Format your code automatically
cargo fmt --all

# Run the linter to detect issues
cargo clippy -- -D warnings
```

---

## Running Tests

KnightOS features an integrated test suite that allows running unit and integration tests inside QEMU.

Before submitting any bug fix or documentation pull request, ensure all existing tests pass:

```bash
# Run the Rust test suite
cargo test

# Run tests with serial port logging output
cargo test -- --nocapture
```

---

## Pull Request and Review Workflow

If you would like to submit a patch (such as a bug fix, clean-up, or a new test), please follow this workflow:

1.  **Fork the repository** and create a new branch from `master`:
    ```bash
    git checkout -b feature/my-cool-improvement
    ```
2.  **Commit your changes** using clear and descriptive commit messages.
3.  **Ensure all tests and style checks pass** locally using `cargo fmt` and `cargo test`.
4.  **Push your branch** to your fork:
    ```bash
    git push origin feature/my-cool-improvement
    ```
5.  **Open a Pull Request** against the main `master` branch. Please explain clearly what problem your PR solves and how you tested it.

Note that since this is an educational project, we might discuss the design in the pull request comments before merging to ensure it aligns with the project's learning goals.
