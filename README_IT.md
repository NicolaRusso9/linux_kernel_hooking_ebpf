# Kernel Hooking & Input Interception via eBPF (Rust + Aya)
[![GitHub Stars](https://img.shields.io/github/stars/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=yellow)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=blue)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/network/members)
[![GitHub Watchers](https://img.shields.io/github/watchers/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square&color=orange)](https://github.com/NicolaRusso9/linux_kernel_hooking_ebpf/watchers)
[![License](https://img.shields.io/github/license/NicolaRusso9/linux_kernel_hooking_ebpf?style=flat-square)](LICENSE)

<div align="right">
  <a href="README.md">English</a> | <strong>Italiano</strong>
</div>

---

Questo repository contiene un progetto dimostrativo che utilizza **eBPF** per effettuare operazioni di *kernel hooking* in ambiente Linux, con un demone user‑space scritto in **Rust** tramite il framework **Aya**.  
L’obiettivo è mostrare come intercettare input sensibili a livello kernel in modo moderno, efficiente e osservabile, sfruttando le capacità di tracciamento offerte da eBPF.

> ⚠️ **Nota etica:**  
> Questo progetto è destinato esclusivamente a scopi educativi e di ricerca sulla sicurezza.  
> Non deve essere utilizzato su sistemi non autorizzati.

---

# **📌 Funzionalità principali**

### **✔ Hooking kernel-space tramite eBPF**
Il modulo eBPF aggancia tre funzioni critiche del VFS:

- `vfs_write` → intercetta ciò che l’utente *scrive* (password, comandi, input sensibile)
- `vfs_read` (enter) → cattura l’indirizzo del buffer di lettura
- `vfs_read` (return) → esfiltra i dati letti dal processo

### **✔ Demone user-space in Rust**
Il demone:

- carica il bytecode eBPF,
- aggancia i kprobe/kretprobe,
- riceve eventi tramite perf buffer,
- filtra i processi target,
- salva i dati intercettati su file,
- applica tecniche di offuscamento del nome del processo.

### **✔ Offuscamento del processo**
- Nome iniziale simile a un kernel thread (`[lab-kworker/0]`)
- Rotazione periodica del nome tramite `prctl(PR_SET_NAME)`
- Esecuzione su thread Tokio single-threaded per garantire coerenza

### **✔ Logging automatico**
Gli eventi vengono salvati in:

```
~/Desktop/pwd_hook.log
```

Il percorso del Desktop viene rilevato dinamicamente tramite il crate `directories`.

---

# **📁 Struttura del progetto**

```
linux_kernel_hooking_ebpf/
 ├── ebpf-hook/            # Programma eBPF compilato in bytecode
 │    ├── src/
 │    └── target/bpf/
 ├── user-daemon/          # Demone Rust che carica e gestisce gli hook
 │    ├── src/
 │    └── Cargo.toml
 ├── shared/               # Strutture dati condivise tra kernel e user-space
 └── README.md
```

---

# **Requisiti**

- Linux kernel 5.8+ con supporto eBPF
- Rust 1.75+
- Toolchain nightly per la build del bytecode
- `cargo install bpf-linker`
- LLVM/Clang
- Permessi root

---

# **🔧 Build**

### **1. Compilazione del modulo eBPF**

```bash
cargo +nightly build \
  --package ebpf-hook-pwd \
  --bin ebpf-hook-pwd \
  --target bpfel-unknown-none \
  -Z build-std=core \
  --release
```

Il bytecode verrà generato in:

```
target/bpf/programs/ebpf-hook-pwd/ebpf-hook-pwd.o
```

### **2. Compilazione del demone user-space**

```bash
cd user-daemon
cargo build --release
```

---

# **Esecuzione**

```bash
sudo ./target/release/user-daemon
```

Il demone:

- carica il bytecode,
- aggancia i kprobe,
- avvia la rotazione del nome del processo,
- inizia a ricevere eventi dal kernel.

---

# **Esempio di output**

```
[HOOKING-LAB] uid=1000 pid=2345 command=bash hex=73 75 64 6f ... data=sudo
[HOOKING-LAB] uid=1000 pid=2345 command=ssh hex=70 61 73 73 ... data=password123
```

---

# **Come funziona**

### **1. Hooking kernel-space**
Il modulo eBPF aggancia:

- `vfs_write` per intercettare input dell’utente,
- `vfs_read` (enter/return) per catturare output verso il processo.

### **2. Comunicazione kernel → user-space**
Gli eventi vengono inviati tramite **perf buffer**, uno dei meccanismi più efficienti per trasferire dati dal kernel.

### **3. Demone Rust**
Il demone:

- legge gli eventi,
- li deserializza in strutture Rust,
- applica filtri,
- li salva su file.

### **4. Offuscamento**
Il processo user‑space ruota periodicamente il proprio nome per mimetizzarsi tra i kernel thread.

---

# **Sicurezza e note etiche**

Questo progetto dimostra:

- la potenza di eBPF come strumento di osservabilità,
- i rischi associati a un uso malevolo,
- la necessità di nuove strategie di auditing del kernel.

Non deve essere utilizzato per scopi non autorizzati.

---
