# ternary-circuit

Circuit and logic design with ternary values — three-valued logic gates, truth tables, circuit composition, and minimization under Kleene and Łukasiewicz semantics.

## Why This Exists

Binary logic has two values: true and false. But many real-world reasoning situations involve a third state — unknown, undefined, or don't-care. Three-valued logic (3VL) has been studied since Łukasiewicz and Kleene, and is used in SQL `NULL` handling, hardware verification (X states in simulation), and multi-valued logic synthesis.

**ternary-circuit** provides a complete three-valued logic toolkit: gate evaluation under two semantics, truth table generation, compositional circuit building, and double-negation minimization. Every gate operates on `Trit` values (False/Unknown/True), making it suitable for logic simulation, verification, and ternary arithmetic circuits.

## Core Concepts

| Type | Meaning |
|---|---|
| `Trit` | Ternary logic value: `False` (-1), `Unknown` (0), `True` (+1) |
| `TernaryGate` | Gate type: And, Or, Not, Xor, Nand, Nor, Imp |
| `LogicSystem` | Semantics: `Kleene` (strong) or `Lukasiewicz` (Ł-logic) |
| `TernaryCircuit` | Compositional circuit with primary inputs and gate instances |

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-circuit = "0.1"
```

```rust
use ternary_circuit::*;

fn main() {
    let sys = LogicSystem::Kleene;

    // Basic gate evaluation
    let and_result = sys.eval_gate(TernaryGate::And, &[Trit::True, Trit::Unknown]);
    assert_eq!(and_result, Trit::Unknown);

    // Build a circuit: (A AND B) OR (NOT A)
    let mut circuit = TernaryCircuit::new(2); // 2 primary inputs
    circuit.add_gate(TernaryGate::And, vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
    circuit.add_gate(TernaryGate::Not, vec![GateInput::Primary(0)], 3);
    circuit.add_gate(TernaryGate::Or, vec![GateInput::Gate(2), GateInput::Gate(3)], 4);

    let result = circuit.evaluate(&[Trit::False, Trit::True], sys);
    println!("Output: {:?}", result[&4]); // True (False AND True = False, NOT False = True, False OR True = True)

    // Generate truth table for XOR
    let table = truth_table(TernaryGate::Xor, LogicSystem::Kleene, 2);
    for (inputs, output) in &table {
        println!("{:?} → {:?}", inputs, output);
    }
}
```

## API Overview

### Trit
- `False` (-1), `Unknown` (0), `True` (+1)
- `value() → i32`, `from_i32(v)` — numeric conversion
- `is_true()`, `is_false()`, `is_unknown()` — predicates

### TernaryGate
- `And`, `Or`, `Not`, `Xor`, `Nand`, `Nor`, `Imp` (implication)

### LogicSystem
- `Kleene` — strong Kleene three-valued logic (unknown propagates)
- `Lukasiewicz` — Łukasiewicz logic (Unknown → Unknown = True)
- `eval_gate(gate, inputs) → Trit` — evaluate under chosen semantics

### Truth Tables
- `truth_table(gate, system, arity) → Vec<(Vec<Trit>, Trit)>` — complete 3^arity table

### TernaryCircuit
- `new(primary_inputs)` — create circuit
- `add_gate(gate, inputs, output_id)` — add gate instance
- `evaluate(inputs, system) → HashMap<usize, Trit>` — run circuit
- `GateInput::Primary(id)` / `GateInput::Gate(id)` — wire to inputs or other gates

### Minimization
- `minimize_circuit(circuit) → TernaryCircuit` — eliminate double-negation patterns

## How It Works

**Kleene logic** treats `Unknown` as a propagating uncertainty: AND with Unknown returns Unknown unless another input is False (which dominates); OR with Unknown returns Unknown unless another input is True. Implication is defined as ¬A ∨ B. This matches the "strong Kleene" interpretation used in program verification.

**Łukasiewicz logic** differs primarily in implication: `Unknown → Unknown = True` (where Kleene would give Unknown). This matches the original Łukasiewicz three-valued logic Ł₃, which is important in modal logic and paraconsistent reasoning.

**Circuit evaluation** processes gates in insertion order (acyclic assumption), building a value map from primary inputs through each gate. Each gate reads its input values (from primary inputs or previous gate outputs), evaluates using the chosen logic system, and stores the result. The final value map contains all intermediate and output signals.

**Minimization** detects NOT → NOT patterns (double negation) and replaces them with a direct wire to the original input, removing both gates from the circuit.

## Use Cases

- **Hardware verification** — simulate digital circuits with X (unknown) states during power-up or metastability, using Kleene semantics
- **SQL NULL reasoning** — model three-valued SQL logic (True/False/NULL) for query optimization and correctness checking
- **Ternary arithmetic** — build adders, multipliers, and comparators on balanced ternary representation {-1, 0, +1}

## Ecosystem

Part of the **SuperInstance** ternary computing ecosystem:

- [`ternary`](https://crates.io/crates/ternary) — core trit types and balanced ternary arithmetic
- [`ternary-circuit`](https://crates.io/crates/ternary-circuit) — this crate
- [`ternary-chaos`](https://crates.io/crates/ternary-chaos) — chaos and nonlinear dynamics for ternary maps
- [`ternary-constraint`](https://crates.io/crates/ternary-constraint) — constraint satisfaction for ternary variables
- [`ternary-control`](https://crates.io/crates/ternary-control) — ternary control theory

## License

MIT
