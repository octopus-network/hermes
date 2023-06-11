use core::fmt::{Display, Error as FmtError, Formatter};

use crate::event::monitor::EventBatch;
use ibc_relayer_types::clients::ics10_grandpa::header::Header as GPheader;
use ibc_relayer_types::{core::ics02_client::events::NewBlock, Height};

/// A command for a [`WorkerHandle`](crate::worker::WorkerHandle).
#[derive(Debug, Clone)]
pub enum WorkerCmd {
    /// A batch of packet events need to be relayed
    IbcEvents { batch: EventBatch },

    /// A new block has been committed
    NewBlock { height: Height, new_block: NewBlock },

    /// Trigger a pending packets clear
    ClearPendingPackets,

    /// A beefy msg need to be relayed
    Beefy { header: GPheader },
}

impl Display for WorkerCmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            WorkerCmd::IbcEvents { batch } => {
                write!(f, "IbcEvents batch from {}: ", batch.chain_id)?;
                for e in &batch.events {
                    write!(f, "{e}; ")?;
                }
                write!(f, "batch Height: {}", batch.height)
            }
            WorkerCmd::NewBlock { height, new_block } => {
                write!(f, "NewBlock({height}, {new_block})")
            }
            WorkerCmd::ClearPendingPackets => write!(f, "CleaPendingPackets"),
            WorkerCmd::Beefy { header } => {
                write!(f, "beefy: {:?}", header)
            }
        }
    }
}
