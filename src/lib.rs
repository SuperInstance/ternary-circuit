#![forbid(unsafe_code)]

//! Circuit and logic design with ternary values.

use std::collections::HashMap;

/// Ternary logic value: False (-1), Unknown (0), True (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    False = -1,
    Unknown = 0,
    True = 1,
}

impl Trit {
    pub fn value(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Trit::False),
            0 => Some(Trit::Unknown),
            1 => Some(Trit::True),
            _ => None,
        }
    }

    pub fn is_true(&self) -> bool {
        *self == Trit::True
    }

    pub fn is_false(&self) -> bool {
        *self == Trit::False
    }

    pub fn is_unknown(&self) -> bool {
        *self == Trit::Unknown
    }
}

/// Ternary gate types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryGate {
    And,
    Or,
    Not,
    Xor,
    Nand,
    Nor,
    Imp,
}

/// Logic system for 3-valued logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicSystem {
    Kleene,
    Lukasiewicz,
}

impl LogicSystem {
    /// Evaluate a gate under this logic system.
    pub fn eval_gate(&self, gate: TernaryGate, inputs: &[Trit]) -> Trit {
        match gate {
            TernaryGate::Not => {
                assert_eq!(inputs.len(), 1);
                match inputs[0] {
                    Trit::True => Trit::False,
                    Trit::False => Trit::True,
                    Trit::Unknown => Trit::Unknown,
                }
            }
            TernaryGate::And => {
                inputs.iter().fold(Trit::True, |acc, &t| self.min(acc, t))
            }
            TernaryGate::Or => {
                inputs.iter().fold(Trit::False, |acc, &t| self.max(acc, t))
            }
            TernaryGate::Xor => {
                inputs.iter().fold(Trit::False, |acc, &t| {
                    match (acc, t) {
                        (Trit::Unknown, _) | (_, Trit::Unknown) => Trit::Unknown,
                        _ => if acc == t { Trit::False } else { Trit::True },
                    }
                })
            }
            TernaryGate::Nand => self.eval_gate(TernaryGate::Not, &[self.eval_gate(TernaryGate::And, inputs)]),
            TernaryGate::Nor => self.eval_gate(TernaryGate::Not, &[self.eval_gate(TernaryGate::Or, inputs)]),
            TernaryGate::Imp => {
                assert_eq!(inputs.len(), 2);
                let not_a = self.eval_gate(TernaryGate::Not, &[inputs[0]]);
                match self {
                    LogicSystem::Kleene => self.max(not_a, inputs[1]),
                    LogicSystem::Lukasiewicz => {
                        match (inputs[0], inputs[1]) {
                            (Trit::False, _) => Trit::True,
                            (_, Trit::True) => Trit::True,
                            (Trit::True, Trit::False) => Trit::False,
                            (Trit::Unknown, Trit::Unknown) => Trit::True, // Łukasiewicz: 0→0 = 1
                            (Trit::True, Trit::Unknown) => Trit::Unknown,
                            (Trit::Unknown, Trit::False) => Trit::Unknown,
                            (Trit::False, _) | (_, Trit::True) => Trit::True,
                            // unreachable with exhaustive patterns above
                        }
                    }
                }
            }
        }
    }

    fn min(&self, a: Trit, b: Trit) -> Trit {
        if a.value() <= b.value() { a } else { b }
    }

    fn max(&self, a: Trit, b: Trit) -> Trit {
        if a.value() >= b.value() { a } else { b }
    }
}

/// Generate a truth table for a gate.
pub fn truth_table(gate: TernaryGate, system: LogicSystem, arity: usize) -> Vec<(Vec<Trit>, Trit)> {
    let values = [Trit::False, Trit::Unknown, Trit::True];
    let total = 3usize.pow(arity as u32);
    let mut table = Vec::new();
    for i in 0..total {
        let mut inputs = Vec::new();
        let mut idx = i;
        for _ in 0..arity {
            inputs.push(values[idx % 3]);
            idx /= 3;
        }
        let output = system.eval_gate(gate, &inputs);
        table.push((inputs, output));
    }
    table
}

/// A gate instance in a circuit.
#[derive(Debug, Clone)]
pub struct GateInstance {
    pub gate: TernaryGate,
    pub inputs: Vec<GateInput>,
    pub output_id: usize,
}

/// Input to a gate: either a primary input or a gate output.
#[derive(Debug, Clone)]
pub enum GateInput {
    Primary(usize),
    Gate(usize),
}

/// A ternary circuit.
pub struct TernaryCircuit {
    pub primary_inputs: usize,
    pub gates: Vec<GateInstance>,
}

impl TernaryCircuit {
    pub fn new(primary_inputs: usize) -> Self {
        Self { primary_inputs, gates: Vec::new() }
    }

    pub fn add_gate(&mut self, gate: TernaryGate, inputs: Vec<GateInput>, output_id: usize) {
        self.gates.push(GateInstance { gate, inputs, output_id });
    }

    /// Evaluate the circuit with given inputs.
    pub fn evaluate(&self, inputs: &[Trit], system: LogicSystem) -> HashMap<usize, Trit> {
        let mut values: HashMap<usize, Trit> = HashMap::new();
        for (i, &v) in inputs.iter().enumerate() {
            values.insert(i, v);
        }
        for gate_inst in &self.gates {
            let gate_inputs: Vec<Trit> = gate_inst.inputs.iter().map(|inp| {
                match inp {
                    GateInput::Primary(id) => *values.get(id).unwrap_or(&Trit::Unknown),
                    GateInput::Gate(id) => *values.get(id).unwrap_or(&Trit::Unknown),
                }
            }).collect();
            let output = system.eval_gate(gate_inst.gate, &gate_inputs);
            values.insert(gate_inst.output_id, output);
        }
        values
    }
}

/// Circuit minimization: remove redundant NOT gates (double negation).
pub fn minimize_circuit(circuit: &TernaryCircuit) -> TernaryCircuit {
    let mut minimized = TernaryCircuit::new(circuit.primary_inputs);
    // Find NOT→NOT pairs
    let not_gates: Vec<&GateInstance> = circuit.gates.iter()
        .filter(|g| g.gate == TernaryGate::Not)
        .collect();

    let mut skip_outputs: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut replacements: HashMap<usize, GateInput> = HashMap::new();

    for g1 in &not_gates {
        if skip_outputs.contains(&g1.output_id) {
            continue;
        }
        if let Some(GateInput::Gate(g1_in)) = g1.inputs.first() {
            // Find if g1_in is output of another NOT
            if let Some(g0) = not_gates.iter().find(|g| g.output_id == *g1_in) {
                if let Some(GateInput::Primary(p)) = g0.inputs.first() {
                    // Double negation: replace g1's output with p
                    replacements.insert(g1.output_id, GateInput::Primary(*p));
                    skip_outputs.insert(g0.output_id);
                    skip_outputs.insert(g1.output_id);
                }
            }
        }
    }

    for gate in &circuit.gates {
        if skip_outputs.contains(&gate.output_id) {
            continue;
        }
        let mut new_inputs = gate.inputs.clone();
        for inp in &mut new_inputs {
            if let GateInput::Gate(id) = inp {
                if let Some(repl) = replacements.get(id) {
                    *inp = repl.clone();
                }
            }
        }
        minimized.add_gate(gate.gate, new_inputs, gate.output_id);
    }

    minimized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_values() {
        assert_eq!(Trit::False.value(), -1);
        assert_eq!(Trit::Unknown.value(), 0);
        assert_eq!(Trit::True.value(), 1);
    }

    #[test]
    fn test_trit_predicates() {
        assert!(Trit::True.is_true());
        assert!(Trit::False.is_false());
        assert!(Trit::Unknown.is_unknown());
    }

    #[test]
    fn test_kleene_and() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::True, Trit::True]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::True, Trit::False]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::True, Trit::Unknown]), Trit::Unknown);
    }

    #[test]
    fn test_kleene_or() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Or, &[Trit::False, Trit::False]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::Or, &[Trit::True, Trit::False]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Or, &[Trit::Unknown, Trit::False]), Trit::Unknown);
    }

    #[test]
    fn test_not() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Not, &[Trit::True]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::Not, &[Trit::False]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Not, &[Trit::Unknown]), Trit::Unknown);
    }

    #[test]
    fn test_nand() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Nand, &[Trit::True, Trit::True]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::Nand, &[Trit::False, Trit::False]), Trit::True);
    }

    #[test]
    fn test_nor() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Nor, &[Trit::False, Trit::False]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Nor, &[Trit::True, Trit::True]), Trit::False);
    }

    #[test]
    fn test_xor() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Xor, &[Trit::True, Trit::False]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Xor, &[Trit::True, Trit::True]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::Xor, &[Trit::Unknown, Trit::True]), Trit::Unknown);
    }

    #[test]
    fn test_kleene_implication() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Imp, &[Trit::False, Trit::False]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Imp, &[Trit::True, Trit::False]), Trit::False);
        assert_eq!(sys.eval_gate(TernaryGate::Imp, &[Trit::True, Trit::True]), Trit::True);
    }

    #[test]
    fn test_lukasiewicz_implication() {
        let sys = LogicSystem::Lukasiewicz;
        assert_eq!(sys.eval_gate(TernaryGate::Imp, &[Trit::Unknown, Trit::Unknown]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::Imp, &[Trit::True, Trit::False]), Trit::False);
    }

    #[test]
    fn test_truth_table_size() {
        let table = truth_table(TernaryGate::And, LogicSystem::Kleene, 2);
        assert_eq!(table.len(), 9); // 3^2
    }

    #[test]
    fn test_truth_table_not() {
        let table = truth_table(TernaryGate::Not, LogicSystem::Kleene, 1);
        assert_eq!(table.len(), 3);
        assert_eq!(table[0].1, Trit::True); // NOT False = True
        assert_eq!(table[2].1, Trit::False); // NOT True = False
    }

    #[test]
    fn test_circuit_simple() {
        let mut c = TernaryCircuit::new(2);
        c.add_gate(TernaryGate::And, vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
        let result = c.evaluate(&[Trit::True, Trit::True], LogicSystem::Kleene);
        assert_eq!(result[&2], Trit::True);
    }

    #[test]
    fn test_circuit_chained() {
        let mut c = TernaryCircuit::new(1);
        c.add_gate(TernaryGate::Not, vec![GateInput::Primary(0)], 1);
        c.add_gate(TernaryGate::Not, vec![GateInput::Gate(1)], 2);
        let result = c.evaluate(&[Trit::True], LogicSystem::Kleene);
        assert_eq!(result[&2], Trit::True); // double negation
    }

    #[test]
    fn test_minimize_double_negation() {
        let mut c = TernaryCircuit::new(1);
        c.add_gate(TernaryGate::Not, vec![GateInput::Primary(0)], 1);
        c.add_gate(TernaryGate::Not, vec![GateInput::Gate(1)], 2);
        let min = minimize_circuit(&c);
        assert!(min.gates.is_empty()); // both NOTs removed
    }

    #[test]
    fn test_minimize_preserves_useful() {
        let mut c = TernaryCircuit::new(2);
        c.add_gate(TernaryGate::And, vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
        c.add_gate(TernaryGate::Not, vec![GateInput::Gate(2)], 3);
        let min = minimize_circuit(&c);
        assert_eq!(min.gates.len(), 2);
    }

    #[test]
    fn test_trit_from_i32() {
        assert_eq!(Trit::from_i32(-1), Some(Trit::False));
        assert_eq!(Trit::from_i32(5), None);
    }

    #[test]
    fn test_circuit_with_unknown() {
        let mut c = TernaryCircuit::new(2);
        c.add_gate(TernaryGate::And, vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
        let result = c.evaluate(&[Trit::True, Trit::Unknown], LogicSystem::Kleene);
        assert_eq!(result[&2], Trit::Unknown);
    }

    #[test]
    fn test_or_with_all_unknown() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::Or, &[Trit::Unknown, Trit::Unknown]), Trit::Unknown);
    }

    #[test]
    fn test_and_identity() {
        let sys = LogicSystem::Kleene;
        assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::True]), Trit::True);
        assert_eq!(sys.eval_gate(TernaryGate::And, &[Trit::False]), Trit::False);
    }

    #[test]
    fn test_circuit_or_gate() {
        let mut c = TernaryCircuit::new(2);
        c.add_gate(TernaryGate::Or, vec![GateInput::Primary(0), GateInput::Primary(1)], 2);
        let result = c.evaluate(&[Trit::False, Trit::True], LogicSystem::Kleene);
        assert_eq!(result[&2], Trit::True);
    }
}
