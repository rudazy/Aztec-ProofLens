# Aztec ProofLens

**The Diagnostic Layer for the Private Web**

Aztec ProofLens is a comprehensive suite designed to provide transparency, observability, and deep diagnostics for the Aztec Network Zero-Knowledge proof system.

## Project Pillars

1. **The Judge (Verifier)**: Mathematical confirmation of proof validity utilizing the Honk and UltraHonk proof systems.
2. **The Surgeon (Diagnostics)**: Precise mapping of failed gates back to Noir source code constraints for streamlined debugging.
3. **The Architect (Visualizer)**: Detailed mapping of the recursive proof hierarchy, including Private, Tube, and Root proofs.
4. **The Vault (Registry)**: A historical database maintaining protocol-wide Verification Keys (VKs) for consistency across network upgrades.

## Technical Specifications

- **Engine**: Rust (leveraging Barretenberg cryptographic bindings)
- **Circuit Logic**: Noir
- **Distribution**: Command Line Interface (CLI) and WebAssembly (Wasm)
