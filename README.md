# 🐾 FemtoClaw Claws (Standard Capabilities)

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

The **FemtoClaw Claws** library provides the standard set of system capabilities used by the runtime. These modules implement the **Capability Claw Specification (FC-04)** and operate under strict authorization from the policy engine.

---

## 🛡️ Capability Security Model

Every "Claw" is a bounded system interface. Unlike raw system calls, Claws are:
1.  **Validated**: Input arguments must match the registered JSON schema.
2.  **Authorized**: Every execution requires an explicit `Allow` result from the Policy Engine.
3.  **Audited**: Both the request and the result are recorded in the tamper-evident execution log.

---

## 🧱 Standard Capability Set

| Claw | Identifier | Description | Invariants |
|------|------------|-------------|------------|
| **Shell** | `shell` | Secure process execution. | Argv-only; no shell interpolation or `sh -c`. |
| **Filesystem** | `fs` | Bounded file operations. | Absolute paths only; strict size limits on read/write. |
| **Network** | `net` | Controlled HTTP/TCP requests. | Strict timeout enforcement and domain allowlisting. |
| **Process** | `process` | Lifecycle management. | Limited to child processes spawned by the runtime. |

---

## 🚀 Usage in Runtime

Capabilities are registered with the `Agent` during initialization:

```rust
let mut agent = Agent::new(config)?;

// Claws are automatically registered in the default industrial profile
agent.execute_tool("shell", json!({
    "bin": "echo",
    "argv": ["hello world"]
})).await?;
```

---

## 📄 Related Specifications
- **[FC-04: Capability Claw Specification](../femtoclaw-spec/04-FemtoClaw_Capability_Claw_Specification.md)**
- **[FC-05: Capability Authorization](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md)**

Copyright © 2026 FemtoClaw Project.
