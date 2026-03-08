# 🦀 Rust_100: The BareMetal Journey

Welcome to **Rust_100**. This repository is a technical log of my transition from C-based kernel engineering to memory-safe systems programming in Rust. 

The goal is to master the language over 100 modules, moving from basic syntax to building a custom hypervisor.



---

## 🗺️ The 10-Phase Roadmap

### 🟢 Phase 1-2: Foundations & Ownership (Videos 1-20)
* **Focus:** Toolchain, Memory Layout (Stack vs Heap), and Move Semantics.
* **Key Concept:** The "Aha!" moment of the Borrow Checker.
* **Featured Project:** `Pulse-CLI` — A real-time system monitor parsing `/proc/stat`.

### 🔵 Phase 3-4: Logic, Enums & Collections (Videos 21-40)
* **Focus:** Robust error handling (`Result`/`Option`) and high-performance data structures.
* **Key Concept:** Zero-cost abstractions and exhaustive pattern matching.
* **Featured Project:** `Mem-Inspect` — Reading raw memory bytes from a running process.

### 🟡 Phase 5-6: Lifetimes & Smart Pointers (Videos 41-60)
* **Focus:** Mastering `'a` lifetimes, `Box<T>`, and `Rc<T>`.
* **Key Concept:** Manual memory control within a safe compiler boundary.
* **Featured Project:** `Safe-FD` — A safety wrapper for Linux File Descriptors.

### 🟠 Phase 7-8: Concurrency & Unsafe Rust (Videos 61-80)
* **Focus:** Fearless concurrency (`Arc`/`Mutex`) and FFI (Calling C from Rust).
* **Key Concept:** Thread safety at the type level and raw pointer manipulation.
* **Featured Project:** `Bare-Driver` — A minimal character device driver for the Linux Kernel.

### 🔴 Phase 9-10: Async & Virtualization (Videos 81-100)
* **Focus:** Macros, Async/Await, and Bare-Metal development.
* **Key Concept:** Building custom executors and interacting with KVM/vGPU.
* **Featured Project:** `Hyper-AI` — Running an AI model inside a custom guest kernel.

---

## 📂 Repository Structure

| Module | Topic | Project | Status |
| :--- | :--- | :--- | :--- |
| **Chap 1** | Toolchain & Hello World | Dissecting Binaries | ✅ |
| **Chap 11** | Ownership Rules | `Pulse-CLI` | 🚧 |
| **Chap 26** | Memory Mastery | `Mem-Inspect` | 📅 |
| **Chap 118** | Linux Drivers | `Bare-Driver` | 📅 |
| **Chap 150** | Virtualization | `Hyper-AI` | 🚀 |

---

## 🛠 Tech Stack
* **Language:** Rust (Stable/Nightly)
* **Host:** Linux (KVM / QEMU / VFIO)
* **Visuals:** [Manim](https://www.manim.community/) (For memory layout animations)

---

## 📺 BareMetal - The Series
Each chapter in this repo corresponds to a video on the **BareMetal** YouTube channel. 
> "Training like a Saiyan in the Hyperbolic Time Chamber of the Rust Compiler."

---

## 👤 About Me
I am a **System Software Engineer** in NVIDIA. I hold an **MTech from IIT Guwahati** and spend my time bridging the gap between hardware and high-level safety.

* **YouTube:** [BareMetal Link]
