use crate::identity::create_identity;
use crate::types::*;
use candid::{Decode, Encode};

pub async fn call_args_is_string_function(
    canister_id: &str,
    method_name: &str,
    arg: String, // 我们使用String来代替原来的Vec<u8>
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = if is_mainnet { MAIN_NET } else { LOCAL_NET };
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_identity(create_identity())
        .build()
        .expect("should work");

    if !is_mainnet {
        agent.fetch_root_key().await?;
    }

    let canister_id = ic_cdk::export::Principal::from_text(canister_id)?;

    let mut update_builder =
        ic_agent::agent::UpdateBuilder::new(&agent, canister_id, method_name.to_string());
    let update_builder_with_args = update_builder.with_arg(&Encode!(&arg)?); // 将参数arg使用Encode!宏转化为字节序列

    let response = update_builder_with_args.call_and_wait().await?;
    Ok(response)
}

async fn update_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = if is_mainnet { MAIN_NET } else { LOCAL_NET };
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_identity(create_identity())
        .build()
        .expect("should work");

    if !is_mainnet {
        agent.fetch_root_key().await?;
    }

    let canister_id = ic_cdk::export::Principal::from_text(canister_id)?;

    let mut update_builder =
        ic_agent::agent::UpdateBuilder::new(&agent, canister_id, method_name.to_string());
    let update_builder_with_args = update_builder.with_arg(&Encode!(&args)?);

    let response = update_builder_with_args.call_and_wait().await?;
    Ok(response)
}

async fn update_ic_and_get_vec(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<VecResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), VecResult)?;

    Ok(response)
}

async fn update_ic_and_get_nothing(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<NullResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), NullResult)?;

    Ok(response)
}

async fn update_ic_and_get_smstate(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<SmStateResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), SmStateResult)?;

    Ok(response)
}

async fn update_ic_and_get_proofs(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<ProofsResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), ProofsResult)?;

    Ok(response)
}

pub(crate) async fn send_msg_for_vec(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let result = update_ic_and_get_vec(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        VecResult::Ok(smheader) => Ok(smheader),
        VecResult::Err(e) => Err(e.into()),
    }
}

pub(crate) async fn send_msg_for_smstate(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<SmState, Box<dyn std::error::Error>> {
    let result = update_ic_and_get_smstate(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        SmStateResult::Ok(state) => Ok(state),
        SmStateResult::Err(e) => Err(e.into()),
    }
}

pub(crate) async fn send_msg_for_proofs(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Proofs, String> {
    let result = update_ic_and_get_proofs(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        ProofsResult::Ok(proofs) => Ok(proofs),
        ProofsResult::Err(e) => Err(e),
    }
}

pub(crate) async fn send_msg(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<(), String> {
    let result = update_ic_and_get_nothing(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        NullResult::Ok(_) => Ok(()),
        NullResult::Err(e) => Err(e),
    }
}
