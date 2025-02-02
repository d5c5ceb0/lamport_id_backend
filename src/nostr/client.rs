use crate::common::error::AppResult;
use nostr::event::{Event, EventId, UnsignedEvent};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NostrClient {
    pub signer: Keys,   // The cryptographic keys used for signing events.
    pub client: Client, // The underlying Nostr SDK client.
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

    //get public key from signer
    pub fn get_pub_key(&self) -> PublicKey {
        self.signer.public_key()
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
        tracing::info!("Signed event: {:?}", signed);
        self.send_event(signed).await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LamportType {
    Create,
    Invite,
    Bind,
    Vote,
    Voting,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LamportBinding {
    pub pubkey: PublicKey,
    pub kind: Kind,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub lamport_type: Option<LamportType>,
}

impl From<LamportBinding> for UnsignedEvent {
    fn from(lamport: LamportBinding) -> Self {
        let tags: Vec<Tag> = lamport
            .tags
            .iter()
            .map(|tag| Tag::parse(tag).unwrap())
            .collect::<Vec<Tag>>();
        EventBuilder::new(lamport.kind, lamport.content)
            .tags(tags)
            .build(lamport.pubkey)

    }
}

impl LamportBinding {
    pub fn new(
        pubkey: PublicKey,
        kind: Kind,
        content: &str,
        lamport_type: Option<LamportType>,
    ) -> Self {
        Self {
            pubkey,
            kind,
            tags: Default::default(),
            content: content.to_owned(),
            lamport_type: lamport_type,
        }
    }

    pub fn encode(&self) -> AppResult<String> {
        Ok(serde_json::to_string(self)?)
    }

    //pub fn add_tag(&mut self, tag: &str) {
    //    self.tags.push(tag.to_owned());
    //}

    pub fn new_kind2321(pubkey: PublicKey, lamport_id: &str, twitter: &str) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2321),
            tags: vec![
                //format!("LamportID = {}", lamport_id),
                //format!("Twitter = {}", twitter),
                //format!("lmport_type = {:?}", LamportType::Create),
                vec!["LamportID".to_string(), lamport_id.to_string()],
                vec!["Twitter".to_string(), twitter.to_string()],
                vec!["lmport_type".to_string(), format!("{:?}", LamportType::Create)],
            ],
            content: format!("LamportID:{}", lamport_id),
            lamport_type: Some(LamportType::Create),
        }
    }

    pub fn new_kind2322(pubkey: PublicKey, lamport_id: &str, address: &str, sig: &str) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2322),
            tags: vec![
                //vec!["LamportID", lamport_id],
                //vec!["Address", address],
                //vec!["sig", sig],
                //vec!["lmport_type", format!("{:?}", LamportType::Bind)],
                vec!["LamportID".to_string(), lamport_id.to_string()],
                vec!["Address".to_string(), address.to_string()],
                vec!["sig".to_string(), sig.to_string()],
                vec!["lmport_type".to_string(), format!("{:?}", LamportType::Bind)],
            ],
            content: format!("LamportID:{} bind address:{}", lamport_id, address),
            lamport_type: Some(LamportType::Bind),
        }
    }

    pub fn new_kind2323(
        pubkey: PublicKey,
        lamport_id: &str,
        project: &str,
        invitee: &str,
        link: &str,
    ) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(2323),
            tags: vec![
                //format!("LamportID = {}", lamport_id),
                //format!("p = {}", project),
                //format!("Invitee = {}", invitee),
                //format!("lmport_type = {:?}", LamportType::Invite),
                //"i =invite".to_string(),
                vec!["LamportID".to_string(), lamport_id.to_string()],
                vec!["p".to_string(), project.to_string()],
                vec!["Invitee".to_string(), invitee.to_string()],
                vec!["lmport_type".to_string(), format!("{:?}", LamportType::Invite)],
                vec!["i".to_string(), "invite".to_string()],
            ],
            content: format!("{} Invite {}, Link:{}", lamport_id, invitee, link),
            lamport_type: Some(LamportType::Invite),
        }
    }

    pub fn new_kind_vote(
        pubkey: PublicKey,
        lamport_id: &str,
        vote_id: &str,
        title: &str,
        content: &str,
        start_time: &str,
        end_time: &str,
        options: &str,
        sig: &str,
    ) -> Self {
        Self {
            pubkey,
            kind: Kind::Custom(1),
            tags: vec![
                vec!["LamportID".to_string(), lamport_id.to_string()],
                vec!["vote_id".to_string(), vote_id.to_string()],
                vec!["title".to_string(), title.to_string()],
                vec!["content".to_string(), content.to_string()],
                vec!["start_time".to_string(), start_time.to_string()],
                vec!["end_time".to_string(), end_time.to_string()],
                vec!["options".to_string(), options.to_string()],
                vec!["sig".to_string(), sig.to_string()],
            ],
            content: format!("{} Vote {}, Title:{}", lamport_id, vote_id, title),
            lamport_type: Some(LamportType::Vote),
        }
    }

}
