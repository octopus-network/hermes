use super::MyConfig;
use serde::{Deserialize, Serialize};
use subxt::rpc::Subscription;
use subxt::rpc_params;
use subxt::OnlineClient;

/// An encoded signed commitment proving that the given header has been finalized.
/// The given bytes should be the SCALE-encoded representation of a
/// `beefy_primitives::SignedCommitment`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignedCommitment(pub sp_core::Bytes);

/// Subscribe to beefy justifications.
pub async fn subscribe_beefy_justifications(
    client: OnlineClient<MyConfig>,
) -> Result<Subscription<SignedCommitment>, subxt::error::Error> {
    let subscription = client
        .rpc()
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await?;
    Ok(subscription)
}
