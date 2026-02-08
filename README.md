---
# Kernel Hooking & Input Interception via eBPF (Rust + Aya)
[![GitHub Stars](https://img.shields.io/github/stars/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=yellow)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=blue)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/network/members)
[![GitHub Watchers](https://img.shields.io/github/watchers/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=orange)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/watchers)
[![License](https://img.shields.io/github/license/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square)](LICENSE)

<div align="right">
<strong>English</strong> | <a href="README_IT.md">Italiano</a>
</div>

---

This repository contains a demonstration project that uses **eBPF** to perform kernel hooking operations in a Linux environment, with a user-space daemon written in **Rust** using the **Aya** framework.
The goal is to demonstrate how to intercept sensitive kernel-level input in a modern, efficient, and observable way, leveraging the tracing capabilities offered by eBPF.

> ⚠️ **Ethics Note:**
> This project is intended solely for educational and security research purposes.
> It should not be used on unauthorized systems.

---

# **📌 Main Features**

### **✔ Kernel-space Hooking via eBPF**
The eBPF module hooks three critical VFS functions:

- `vfs_write` → intercepts what the user *writes* (passwords, commands, sensitive input)
- `vfs_read` (enter) → captures the address of the read buffer
- `vfs_read` (return) → exfiltrates data read by the process

### **✔ User-space Daemon in Rust**
The daemon:

- loads eBPF bytecode,
- hooks kprobes/kretprobes,
- receives events via the perf buffer,
- filters target processes,
- saves intercepted data to a file,
- applies process name obfuscation techniques.

### **✔ Process obfuscation**
- Initial name similar to a kernel thread (`[lab-kworker/0]`)
- Periodic name rotation via `prctl(PR_SET_NAME)`
- Runs on a single-threaded Tokyo thread to ensure consistency

### **✔ Automatic logging**
Events are saved to:

```
~/Desktop/pwd_hook.log
```

The Desktop path is dynamically detected via the `directories` crate.

---

# **📁 Project Structure**

```
linux_kernel_hooking_ebpf/
├── ebpf-hook/ # eBPF program compiled into bytecode
│ ├── src/
│ └── target/bpf/
├── user-daemon/ # Rust daemon that loads and manages hooks
│ ├── src/
│ └── Cargo.toml
├── shared/ # Data structures shared between kernel and user space
└── README.md
```

---

# **Requirements**

- Linux kernel 5.8+ with eBPF support
- Rust 1.75+
- Nightly toolchain for bytecode build
- `cargo install bpf-linker`
- LLVM/Clang
- Root permissions

---

# **🔧 Build**

### **1. Filling out the eBPF form**

```bash
cargo +nightly build \ 
--package ebpf-hook-pwd \ 
--bin ebpf-hook-pwd \ 
--target bpfel-unknown-none \ 
-Z build-std=core \ 
--release
```

The bytecode will be generated in:

```
target/bpf/programs/ebpf-hook-pwd/ebpf-hook-pwd.o
```

### **2. Compiling the user-space daemon**

```bash
cd user-daemon
cargo build --release
```

---

# **Execution**

```bash
sudo ./target/release/user-daemon
```

The daemon:

- loads the bytecode,
- hooks the kprobes,
- starts process name rotation,
- starts receiving events from the kernel.

---

# **Example Output**

```
[HOOKING-LAB] uid=1000 pid=2345 command=bash hex=73 75 64 6f ... data=sudo
[HOOKING-LAB] uid=1000 pid=2345 command=ssh hex=70 61 73 73 ... data=password123
```

---

# **How ​​it works**

### **1. Kernel-space Hooking**
The eBPF module hooks:

- `vfs_write` to intercept user input,
- `vfs_read` (enter/return) to capture output to the process.

### **2. Kernel → User-space Communication**
Events are sent via **perf buffer**, one of the most efficient mechanisms for transferring data from the kernel.

### **3. Rust Daemon**
The daemon:

- reads events,
- deserializes them into Rust structures,
- applies filters,
- saves them to files.

### **4. Obfuscation**
The user-space process periodically rotates its name to blend in with kernel threads.

---

# **Security and Ethics Notes**

This project demonstrates:

- the power of eBPF as an observability tool,
- the risks associated with malicious use,
- the need for new kernel auditing strategies.

It must not be used for unauthorized purposes.

---
