use super::{
    enforce_constraints, Hasher, HASHER_NODE_INDEX_COL_IDX, HASHER_SELECTOR_COL_RANGE,
    HASHER_STATE_COL_RANGE, NUM_CONSTRAINTS, ONE, ZERO,
};
use crate::{
    trace::chiplets::hasher::{Selectors, LINEAR_HASH, STATE_WIDTH},
    Felt, TRACE_WIDTH,
};
use alloc::vec::Vec;
use rand_utils::rand_array;
use vm_core::chiplets::hasher::apply_round;
use winter_air::EvaluationFrame;

// UNIT TESTS
// ================================================================================================

/// Tests instruction HR, which is executed on all cycles that are not one less than a multiple of
/// eight, and applies a round of the VM's native hash function.
#[test]
fn hash_round() {
    let expected = [ZERO; NUM_CONSTRAINTS];

    let cycle_row_num: usize = 3;
    let current_selectors = [ZERO, LINEAR_HASH[1], LINEAR_HASH[2]];
    let next_selectors = current_selectors;

    let frame = get_test_hashing_frame(current_selectors, next_selectors, cycle_row_num);
    let result = get_constraint_evaluation(frame, cycle_row_num);
    assert_eq!(expected, result);
}

// TEST HELPER FUNCTIONS
// ================================================================================================

/// Returns the result of hasher chiplet's constraint evaluations on the provided frame starting at
/// the specified row.
fn get_constraint_evaluation(
    frame: EvaluationFrame<Felt>,
    cycle_row_num: usize,
) -> [Felt; NUM_CONSTRAINTS] {
    let mut result = [ZERO; NUM_CONSTRAINTS];
    let periodic_values = get_test_periodic_values(cycle_row_num);

    enforce_constraints(&frame, &periodic_values, &mut result, ONE);

    result
}

/// Returns the values from the periodic columns for the specified cycle row.
fn get_test_periodic_values(cycle_row: usize) -> Vec<Felt> {
    // Set the periodic column values.
    let mut periodic_values = match cycle_row {
        0 => vec![ZERO, ZERO, ONE],
        7 => vec![ZERO, ONE, ZERO],
        8 => vec![ONE, ZERO, ZERO],
        _ => vec![ZERO, ZERO, ZERO],
    };

    // Add the RPO round constants for the first 7 rows of the cycle, or pad with zeros.
    if cycle_row == 7 {
        periodic_values.resize(periodic_values.len() + STATE_WIDTH * 2, ZERO);
    } else {
        periodic_values.extend_from_slice(&Hasher::ARK1[cycle_row]);
        periodic_values.extend_from_slice(&Hasher::ARK2[cycle_row]);
    }
    periodic_values
}

/// Returns a valid test frame for a transition where one round of Rescue-XLIX is computed.
fn get_test_hashing_frame(
    current_selectors: Selectors,
    next_selectors: Selectors,
    cycle_row_num: usize,
) -> EvaluationFrame<Felt> {
    let mut current = vec![ZERO; TRACE_WIDTH];
    let mut next = vec![ZERO; TRACE_WIDTH];

    // Set the selectors for the hash operation.
    current[HASHER_SELECTOR_COL_RANGE].copy_from_slice(&current_selectors);
    next[HASHER_SELECTOR_COL_RANGE].copy_from_slice(&next_selectors);

    // Set the starting hasher state.
    let mut state = rand_array();
    current[HASHER_STATE_COL_RANGE].copy_from_slice(&state);

    // Set the hasher state after a single permutation.
    apply_round(&mut state, cycle_row_num);
    next[HASHER_STATE_COL_RANGE].copy_from_slice(&state);

    // Set the node index values to zero for hash computations.
    current[HASHER_NODE_INDEX_COL_IDX] = ZERO;
    next[HASHER_NODE_INDEX_COL_IDX] = ZERO;

    EvaluationFrame::from_rows(current, next)
}
