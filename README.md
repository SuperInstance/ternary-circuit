# ternary-circuit

A ternary logic circuit design library implementing multi-valued logic gates, truth-table generation, circuit composition, and minimization. Supports both **Kleene** (strong Kleene, K₃) and **Łukasiewicz** (Ł₃) three-valued logic systems.

## Why It Matters

Binary logic circuits cannot express uncertainty. In agent fleets, sensor readings, network state, and trust assessments are often unknown rather than definitively true or false. Three-valued logic (3VL) circuits allow hardware and software to propagate uncertainty through computation rather than collapsing it prematurely.

The `Unknown` value (0) represents "insufficient evidence to decide." In Kleene's K₃, this is non-designated — it never causes a tautology — making it suitable for **conservative** decision-making where unknown inputs produce unknown outputs. In Łukasiewicz's Ł₃, certain compositions of Unknown can still yield True, enabling **optimistic** reasoning.

Within the **γ + η = C** framework:

| Symbol | Domain |
|--------|--------|
| γ | `Trit` ∈ {False(−1), Unknown(0), True(+1)} — signal values on circuit wires |
| η | Gate selection and logic system choice (Kleene vs Łukasiewicz) |
| C | Logical constraints: truth-table semantics, consistency rules |

## How It Works

### Three-Valued Logic Systems

#### Kleene's Strong Logic (K₃)

Operations use the **knowledge ordering**: Unknown is between False and True.

| AND | F | U | T | | OR | F | U | T |
|-----|---|---|---| |----|---|---|---|
| **F** | F | F | F | | **F** | F | U | T |
| **U** | F | U | U | | **U** | U | U | T |
| **T** | F | U | T | | **T** | T | T | T |

NOT: F↔T, U→U.

AND is implemented as **min** over the ordering F < U < T, and OR as **max**:

$$\text{AND}(a, b) = \min(a, b), \qquad \text{OR}(a, b) = \max(a, b)$$

where the total order is −1 < 0 < +1.

#### Łukasiewicz Logic (Ł₃)

Differs from K₃ only in **implication**:

$$a \to b = \begin{cases} \text{True} & \text{if } a \leq b \\ \neg a \lor b & \text{otherwise} \end{cases}$$

Crucially, in Ł₃: **U → U = True** (since 0 ≤ 0), whereas in K₃: U → U = U. This makes Ł₃ a paraconsistent logic where certain paradoxes become tractable.

#### Implication Comparison

| (a, b) | K₃: a→b | Ł₃: a→b |
|---------|---------|----------|
| (F, F) | T | T |
| (F, T) | T | T |
| (T, F) | F | F |
| (T, T) | T | T |
| (U, U) | U | **T** |
| (T, U) | U | U |
| (U, F) | U | U |

### XOR in Three-Valued Logic

$$\text{XOR}(a, b) = \begin{cases} \text{Unknown} & \text{if } a = \text{U} \text{ or } b = \text{U} \\ \text{True} & \text{if } a \neq b \\ \text{False} & \text{if } a = b \end{cases}$$

This preserves the epistemic property: if either input is unknown, the result is unknown.

### Truth Table Generation

For a gate of arity *k* over 3 values, the truth table has $3^k$ entries. The library generates these by enumerating all input combinations:

```rust
pub fn truth_table(gate: TernaryGate, system: LogicSystem, arity: usize)
    -> Vec<(Vec<Trit>, Trit)>
```

| Arity | Entries |
|-------|---------|
| 1 | 3 |
| 2 | 9 |
| 3 | 27 |
| *k* | $3^k$ |

### Circuit Model

A `TernaryCircuit` is a DAG of `GateInstance`s:

```
Primary Inputs (Trit[0..n])
        │
        ▼
   ┌─────────┐
   │ Gate 0  │── output_id = n
   └─────────┘
        │
        ▼
   ┌─────────┐
   │ Gate 1  │── output_id = n+1
   └─────────┘
        │
        ▼
    (outputs HashMap<usize, Trit>)
```

Evaluation processes gates in topological order. Each gate reads its inputs (primary or from prior gate outputs) and writes its result to a shared value map.

### Circuit Minimization: Double Negation Elimination

The minimizer identifies **NOT→NOT** pairs where a NOT gate feeds directly into another NOT gate:

$$\neg(\neg x) \equiv x$$

The optimizer replaces the second NOT's output references with the original primary input and removes both gates. This is the three-valued equivalent of double-negation elimination in classical logic, and it holds in both K₃ and Ł₃.

### Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| `eval_gate` | O(k) | k = input count |
| `truth_table` | O(3^k) | Exponential in arity |
| `evaluate` (circuit) | O(g · k_avg) | g = gate count |
| `minimize_circuit` | O(g²) | Pairwise NOT comparison |

## Quick Start

```rust
use ternary_circuit::{Trit, TernaryGate, LogicSystem, TernaryCircuit, GateInput, truth_table};

let sys = LogicSystem::Kleene;

// Basic gates
assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::True, Trit::Unknown]), Trit::Unknown);
assert_eq!(sys.eval_gate(TernaryGate::Or, &[Trit::False, Trit::True]), Trit::True);
assert_eq!(sys.eval_gate(TernaryGate::Not, &[Trit::Unknown]), Trit::Unknown);

// Build a circuit: (A AND B) OR (NOT A)
let mut circuit = TernaryCircuit::new(2);
circuit.add_gate(TernaryGate::And,
    vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
circuit.add_gate(TernaryGate::Not,
    vec![GateInput::Primary(0)], 3);
circuit.add_gate(TernaryGate::Or,
    vec![GateInput::Gate(2), GateInput::Gate(3)], 4);

let result = circuit.evaluate(&[Trit::True, Trit::False], sys);
assert_eq!(result[&4], Trit::True); // (T AND F) OR (NOT T) = U OR F = U... see truth tables

// Generate truth table for 2-input AND
let table = truth_table(TernaryGate::And, LogicSystem::Kleene, 2);
assert_eq!(table.len(), 9); // 3^2
```

## API

### `Trit`

```rust
pub enum Trit { False = -1, Unknown = 0, True = 1 }
```

Methods: `value()`, `from_i32()`, `is_true()`, `is_false()`, `is_unknown()`.

### `TernaryGate`

Variants: `And`, `Or`, `Not`, `Xor`, `Nand`, `Nor`, `Imp`.

### `LogicSystem`

Variants: `Kleene`, `Lukasiewicz`. Method: `eval_gate(gate, inputs) -> Trit`.

### `TernaryCircuit`

| Method | Description |
|--------|-------------|
| `new(primary_inputs)` | Create with *n* primary inputs |
| `add_gate(gate, inputs, output_id)` | Add a gate instance |
| `evaluate(inputs, system)` | Evaluate all gates; returns `HashMap<usize, Trit>` |

### `minimize_circuit(circuit) -> TernaryCircuit`

Eliminates double-negation pairs.

## Architecture Notes

The circuit model uses **unsigned integer IDs** for wire references rather than named signals. This avoids string allocation in hot paths and maps directly to hardware description languages (HDLs). The trade-off is that human readability requires a separate symbol table.

The choice to support both K₃ and Ł₃ as a runtime parameter (rather than compile-time generic) allows circuits to be re-evaluated under different logics without recompilation — important for agents that may need to switch between conservative and optimistic reasoning modes.

The minimizer currently implements only double-negation elimination. A production ternary circuit optimizer would also include: constant propagation (e.g., `AND(x, False) = False`), dead gate elimination, and Quine-McCluskey-style minimization over the $3^k$ truth table.

## References

- **Kleene, S. C.** (1952). *Introduction to Metamathematics*, §64. — Strong three-valued logic (K₃).
- **Łukasiewicz, J.** (1920). "O logice trójwartościowej" (On three-valued logic). *Ruch Filozoficzny*, 5, 170–171. — Original Ł₃.
- **Malinowski, G.** (2001). "Many-Valued Logics." In *The Blackwell Guide to Philosophical Logic*. — Survey of K₃ vs Ł₃ semantics.
- **Miller, D. M., & Thornton, M. A.** (2008). *Multiple Valued Logic: Concepts and Representations*. — Ternary circuit synthesis and minimization.
- **Post, E. L.** (1921). "Introduction to a General Theory of Elementary Propositions." *American Journal of Mathematics*, 43(3), 163–185. — Foundations of many-valued logic.

## License

MIT
