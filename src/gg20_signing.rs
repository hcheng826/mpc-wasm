use anyhow::{anyhow, Context, Result};
use serde_json::Value;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub async fn sign(
    data_to_sign: String,
    local_share: String,
    parties: Vec<u16>,
    address: surf::Url,
    room: String,
) -> Result<String> {
    // let local_share = serde_json::from_slice(&local_share).context("parse local share")?;
    let local_share: Value = serde_json::from_str(&local_share).context("parse local share")?;
    let number_of_parties = parties.len();

    log!("{:?}", local_share);

    Ok("test".to_string())

    // let (i, incoming, outgoing) = join_computation(address.clone(), &format!("{}-offline", room))
    //     .await
    //     .context("join offline computation")?;

    // let incoming = incoming.fuse();

    // tokio::pin!(incoming);
    // tokio::pin!(outgoing);

    // let signing = OfflineStage::new(i, parties, local_share)?;

    // let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
    //     .run()
    //     .await
    //     .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;

    // let (i, incoming, outgoing) = join_computation(address, &format!("{}-online", room))
    //     .await
    //     .context("join online computation")?;

    // tokio::pin!(incoming);
    // tokio::pin!(outgoing);

    // let message = match hex::decode(data_to_sign.clone()) {
    //     Ok(x) => x,
    //     Err(_e) => data_to_sign.as_bytes().to_vec(),
    // };

    // let message = &message[..];

    // let (signing, partial_signature) =
    //     SignManual::new(BigInt::from_bytes(message), completed_offline_stage)?;

    // outgoing
    //     .send(Msg {
    //         sender: i,
    //         receiver: None,
    //         body: partial_signature,
    //     })
    //     .await?;

    // let partial_signatures: Vec<_> = incoming
    //     .take(number_of_parties - 1)
    //     .map_ok(|msg| msg.body)
    //     .try_collect()
    //     .await?;

    // let signature = signing
    //     .complete(&partial_signatures)
    //     .context("online stage failed")?;

    // let signature = serde_json::to_string(&signature).context("serialize signature")?;

    // Ok(signature)
}
