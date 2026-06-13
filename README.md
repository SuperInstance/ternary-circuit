# Ternary Circuit

**Ternary Circuit** implements circuit and logic design with three-valued signals {-1 (False), 0 (Unknown), +1 (True)} — providing ternary gates, logic systems (Kleene and Łukasiewicz), and combinational circuit simulation.

## Why It Matters

Binary logic can't represent uncertainty. But real circuits encounter unknown states: uninitialized signals, metastability, don't-care conditions. Three-valued logic (3VL) handles these natively — 0 means "we don't know yet" rather than forcing a binary choice. Ternary logic also enables higher information density per wire (log₂3 ≈ 1.585 bits/trit) and is the theoretical foundation for ternary processors. This crate implements both Kleene's strong three-valued logic (used in formal verification) and Łukasiewicz's logic (used in fuzzy reasoning).

## How It Works

### Trit Values

```
False (-1):    definitely false
Unknown (0):   truth value undetermined
True (+1):     definitely true
```

### Logic Systems

**Kleene's K₃ (strong Kleene):**
```
AND(a, b) = min(a, b)
OR(a, b)  = max(a, b)
NOT(a)    = -a
XOR(a, b) = |a - b|

Truth table (AND):
         False  Unknown  True
False    False   False   False
Unknown  False   Unknown Unknown
True     False  Unknown  True
```

**Łukasiewicz's L₃:**
```
Implication: a → b = min(1, 1 - a + b)
```

Gate evaluation: **O(1)** per gate. Circuit of G gates: **O(G)** for combinational evaluation.

### Gate Types

```
And, Or, Not, Xor, Nand, Nor, Imp (implication)
```

Each gate: truth table lookup or arithmetic formula. **O(1)** per evaluation.

### Circuit Simulation

A circuit is a DAG of gates:

```
Circuit {
    gates: Vec<TernaryGate>,
    inputs: Vec<usize>,   // gate indices for primary inputs
    outputs: Vec<usize>,  // gate indices for primary outputs
    wires: Vec<(usize, usize)>,  // (source_gate, dest_gate)
}
```

Topological evaluation: compute gates in dependency order. **O(G + W)** for G gates and W wires.

### Implication

Both Kleene and Łukasiewicz define implication differently:

```
Kleene:    a → b = max(-a, b)   = OR(NOT(a), b)
Łukasiewicz: a → b = min(1, -a + b + 1)
```

The Łukasiewicz version is the basis for fuzzy logic controllers.

## Quick Start

```rust
use ternary_circuit::{Trit, TernaryGate, LogicSystem};

let a = Trit::True;
let b = Trit::Unknown;

// Kleene's logic
let result = TernaryGate::And.evaluate(a, b, LogicSystem::Kleene);
assert_eq!(result, Trit::Unknown);  // True AND Unknown = Unknown

let not_a = TernaryGate::Not.evaluate(a, Trit::False, LogicSystem::Kleene);
assert_eq!(not_a, Trit::False);  // NOT True = False
```

## API

| Type | Description |
|------|-------------|
| `Trit` | False (-1), Unknown (0), True (+1) |
| `TernaryGate` | And, Or, Not, Xor, Nand, Nor, Imp |
| `LogicSystem` | Kleene (K₃), Łukasiewicz (L₃) |
| `Circuit` | DAG of gates with input/output wires |

Key methods: `TernaryGate::evaluate(a, b, system)`, `Circuit::evaluate(inputs)`.

## Architecture Notes

Ternary Circuit provides the logic design layer for ternary computation in SuperInstance. In γ + η = C, True (+1) represents γ (growth — affirming a computation), False (-1) represents η (avoidance — rejecting a computation), and Unknown (0) represents the neutral/uncomputed state. The three-valued logic naturally handles partial information in fleet decision-making. Integrates with `ternary-compiler-optimizer` for circuit optimization and `ternary-codes` for error correction.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for ternary computation architecture.


### Truth Tables (Kleene K₃)

**AND** (min):
```
         False  Unknown  True
False    False   False   False
Unknown  False   Unknown Unknown
True     False  Unknown  True
```

**OR** (max):
```
         False  Unknown  True
False    False  Unknown  True
Unknown  Unknown Unknown  True
True     True   True     True
```

**NOT** (negation): False↔True, Unknown→Unknown

### Information Density

Each trit carries log₂(3) ≈ 1.585 bits of information. A 3-wire ternary bus carries the same information as a 5-wire binary bus (3 × 1.585 = 4.755 ≈ 5 bits). This density advantage compounds in large circuits: a 32-trit multiplier is smaller than a 51-bit multiplier for equivalent range.

### Łukasiewicz Implication

```
a → b = min(1, 1 - a + b)    (L₃ system)
```

This is the basis for fuzzy logic controllers and many-valued logics used in industrial control systems.

## References

1. Kleene, S. C. (1952). *Introduction to Metamathematics*. North-Holland. (Three-valued logic)
2. Łukasiewicz, J. (1920). "O logice trójwartościowej." *Ruch Filozoficzny*, 5, 170–171.
3. Hayes, B. (2001). "Third Base." *American Scientist*, 89(6), 490–494.

## License

MIT
