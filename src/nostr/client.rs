use crate::common::error::AppResult;
use nostr_sdk::prelude::*;
use nostr::event::{Event, EventId, UnsignedEvent};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub struct NostrClient {
    signer: Keys,         // The cryptographic keys used for signing events.
    client: Client,       // The underlying Nostr SDK client.
}

impl NostrClient {
    pub async fn new(priv_key: &str, relay: Option<&str>) -> AppResult<Self> {
        let keys = Keys::parse(priv_key)?;
        let opts = Options::new().gossip(true);
        let client_builder = Client::builder().signer(keys.clone()).opts(opts);
        let client = client_builder.build();

        if let Some(url) = relay {
            client.add_relay(url).await?;
        }
        client.connect().await;

        Ok(Self {
            signer: keys,
            client,
        })
    }

    pub async fn sign(&self, event: UnsignedEvent) -> AppResult<Event> {
        Ok(event.sign(&self.signer.clone()).await?)
    }

    pub async fn send_event(&self, event: Event) -> AppResult<EventId> {
        Ok(self.client.send_event(event).await?.id().to_owned())
    }

    pub async fn sign_and_send(&self, msg: &LamportBinding) -> AppResult<EventId> {
        let event: UnsignedEvent = msg.clone().into();
        let signed = event.sign(&self.signer.clone()).await?;
        self.send_event(signed).await
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LamportBinding {
    pub pubkey: PublicKey,
    pub kind: Kind,
    pub tags: Vec<String>,
    pub content: String,
}

impl From<LamportBinding> for UnsignedEvent {
    fn from(lamport: LamportBinding) -> Self {
        EventBuilder::new(lamport.kind, lamport.content)
            .tags(Tag::parse(lamport.tags))
            .build(lamport.pubkey)
    }
}

impl LamportBinding {
    pub fn new(pubkey: PublicKey, kind: Kind, content: &str) -> Self {
        Self {
            pubkey,
            kind,
            tags: Default::default(),
            content: content.to_owned(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_owned());
    }

    pub fn new_kind2321(pubkey: PublicKey, lamport_id: &str, twitter: &str) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2321),
            tags: vec![format!("LamportID = {}", lamport_id), format!("Twitter = {}", twitter)],
            content: format!("LamportID:{}", lamport_id),
        }
    }

    pub fn new_kind2322(pubkey: PublicKey, lamport_id: &str, address: &str, sig: &str) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2322),
            tags: vec![format!("LamportID = {}", lamport_id), format!("Address = {}", address), format!("sig = {}", sig)],
            content: format!("LamportID:{}", lamport_id),
        }
    }

    pub fn new_kind2323(pubkey: PublicKey, lamport_id: &str, project: &str, invitee: &str, link: &str) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2323),
            tags: vec![format!("LamportID = {}", lamport_id), format!("Project = {}", project), format!("Invitee = {}", invitee), format!("Link = {}", link)],
            content: format!("{} Invite {}, Link:{}", lamport_id, invitee, link),
        }
    }
}
