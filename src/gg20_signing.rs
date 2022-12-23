use serde_json::Value;
use std::convert::TryInto;
use anyhow::{anyhow, Context, Result};
use futures::{Sink, Stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use round_based::Msg;
use round_based::async_runtime::AsyncProtocol;

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

    // log!("{:?}", local_share);

    // "::<String>" is a guess below
    let (i, incoming, outgoing) = join_computation::<String>(address.clone(), &format!("{}-offline", room))
        .await
        .context("join offline computation")?;

    Ok("test".to_string())

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


pub async fn join_computation<M>(
    address: surf::Url,
    room_id: &str,
) -> Result<(
    u16,
    impl Stream<Item = Result<Msg<M>>>,
    impl Sink<Msg<M>, Error = anyhow::Error>,
)>
where
    M: Serialize + DeserializeOwned,
{
    log!("join_computation: {:?}", room_id);
    log!("join_computation: {:?}", address);

    let client = SmClient::new(address, room_id).context("construct SmClient")?;

    log!("client");

    // Construct channel of incoming messages
    let incoming = client
        .subscribe()
        .await
        .context("subscribe")?
        .and_then(|msg| async move {
            serde_json::from_str::<Msg<M>>(&msg).context("deserialize message")
        });

    // Obtain party index
    let index = client.issue_index().await.context("issue an index")?;

    // Ignore incoming messages addressed to someone else
    let incoming = incoming.try_filter(move |msg| {
        futures::future::ready(
            msg.sender != index && (msg.receiver.is_none() || msg.receiver == Some(index)),
        )
    });

    // Construct channel of outgoing messages
    let outgoing = futures::sink::unfold(client, |client, message: Msg<M>| async move {
        let serialized = serde_json::to_string(&message).context("serialize message")?;
        client
            .broadcast(&serialized)
            .await
            .context("broadcast message")?;
        Ok::<_, anyhow::Error>(client)
    });

    Ok((index, incoming, outgoing))
}

pub struct SmClient {
    http_client: surf::Client,
}

impl SmClient {
    pub fn new(address: surf::Url, room_id: &str) -> Result<Self> {

        log!("SmClient: {:?}", room_id);
        log!("SmClient: {:?}", address);

        let config = surf::Config::new()
            .set_base_url(address.join(&format!("rooms/{}/", room_id))?)
            .set_timeout(None);

        log!("SmClient: {:?}", config);

        Ok(Self {
            http_client: config.try_into()?,
        })
    }

    pub async fn issue_index(&self) -> Result<u16> {
        let response = self
            .http_client
            .post("issue_unique_idx")
            .recv_json::<IssuedUniqueIdx>()
            .await
            .map_err(|e| e.into_inner())?;
        Ok(response.unique_idx)
    }

    pub async fn broadcast(&self, message: &str) -> Result<()> {
        self.http_client
            .post("broadcast")
            .body(message)
            .await
            .map_err(|e| e.into_inner())?;
        Ok(())
    }

    pub async fn subscribe(&self) -> Result<impl Stream<Item = Result<String>>> {
        let response = self
            .http_client
            .get("subscribe")
            .await
            .map_err(|e| e.into_inner())?;
        let events = async_sse::decode(response);
        Ok(events.filter_map(|msg| async {
            match msg {
                Ok(async_sse::Event::Message(msg)) => Some(
                    String::from_utf8(msg.into_bytes())
                        .context("SSE message is not valid UTF-8 string"),
                ),
                Ok(_) => {
                    // ignore other types of events
                    None
                }
                Err(e) => Some(Err(e.into_inner())),
            }
        }))
    }
}

#[derive(Deserialize, Debug)]
struct IssuedUniqueIdx {
    unique_idx: u16,
}
