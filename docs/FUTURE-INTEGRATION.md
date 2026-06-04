# Future Integration: ternary-circuit

## Current State
Provides ternary logic gate design with `Trit` values, multiple `TernaryGate` types (AND, OR, NOT, XOR, MIN, MAX, CONSENSUS), logic system selection (Kleene, Łukasiewicz), and circuit evaluation.

## Integration Opportunities

### With ternary-logic (Physical Implementation)
`ternary-logic` defines truth tables for three-valued logics. `ternary-circuit` implements those truth tables as gates. Each `LogicSystem` in `ternary-logic` corresponds to a gate implementation in `ternary-circuit`. The `eval_gate()` method IS the physical realization of the logical entailment.

### With ternary-hardware (Gate-Level Design)
`ternary-hardware` defines ALU and register abstractions. `ternary-circuit` defines the gate-level implementation of those abstractions. A `TernaryALU` is built from `TernaryGate` components. The `CONSENSUS` gate is particularly interesting — it resolves three ternary inputs into a consensus value, exactly what room voting needs at the hardware level.

### With ternary-compiler-v2 (Target Hardware)
The compiler generates IR opcodes. `ternary-circuit` defines how those opcodes execute at the gate level. Register transfer level (RTL) design: compiler IR → circuit gates → hardware execution.

## Potential in Mature Systems
In room-as-codespace, ternary circuits are the execution layer for room-local computation. A room's ensign is implemented as a ternary circuit: inputs (room state) → gates (decision logic) → outputs (room actions). On ESP32, these circuits compile to native instructions. On Codespace, they run as software simulation.

## Cross-Pollination Ideas
- CONSENSUS gate as the hardware implementation of ternary-voting
- Circuit minimization as room simplification — fewer gates = lower compute cost
- Ternary XOR as the basis for room change detection — XOR of current and previous state highlights what changed

## Dependencies for Next Steps
- Integration with ternary-logic for logic-to-gate mapping
- Integration with ternary-hardware for gate-to-ALU composition
- Circuit simulation and minimization tools
