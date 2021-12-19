use crossbeam_channel::{Receiver, Select};

pub fn try_recv_multiple<K, T>(rs: &[(K, Receiver<T>)]) -> Option<(&K, T)> {
    // Build a list of operations.
    let mut sel = Select::new();
    for (_, r) in rs {
        if r.len() != 0 {
            tracing::trace!("in recv_multiple: [try_recv_multiple] -- relayer_process_channel_events 13) len: {:?}", r.len());
        }
        sel.recv(r);
    }

    // Complete the selected operation.
    let oper = sel.try_select().ok()?;
    let index = oper.index();

    // Get the receiver who is ready
    let (k, r) = &rs[index];

    // Receive the message
    let result = oper.recv(r).ok()?;

    Some((k, result))
}
