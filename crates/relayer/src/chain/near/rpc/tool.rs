use near_primitives::views::StateItem;
use std::collections::HashMap;

/// Convert `StateItem`s over to a Map<data_key, value_bytes> representation.
/// Assumes key and value are base64 encoded, so this also decodes them.
pub(crate) fn into_state_map(
    state_items: &[StateItem],
) -> anyhow::Result<HashMap<Vec<u8>, Vec<u8>>> {
    let decode = |s: &StateItem| Ok((base64::decode(&s.key)?, base64::decode(&s.value)?));

    state_items.iter().map(decode).collect()
}
