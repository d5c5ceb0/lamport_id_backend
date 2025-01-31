pub const TOKEN_ENDPOINT: &str = "https://api.twitter.com/oauth2/token";
pub const USERINFO_ENDPOINT: &str = "https://api.twitter.com/2/users/me";
pub const AUTH_ENDPOINT: &str = "https://api.x.com/2/users/me";
pub const JWT_SECRET_KEY: &str = "my_secret_key";
pub const JWT_EXPIRATION: i64 = 2880;
pub const REDIS_KEY: &str = "lamport_id";

pub const POINTS_PROPOSAL: &str = "proposal";
pub const POINTS_INVITE: &str = "invite";
pub const POINTS_VOTE: &str = "vote";
pub const POINTS_BINDING: &str = "binding";

pub const POINTS_PROPOSAL_VALUE: i32 = 100;
pub const POINTS_INVITE_VALUE: i32 = 100;
pub const POINTS_VOTE_VALUE: i32 = 10;
pub const POINTS_BINDING_VALUE: i32 = 100;

pub const ENERGY_PROPOSAL: &str = "proposal";
pub const ENERGY_VOTE: &str = "vote";
pub const ENERGY_REGISTER: &str = "register";
pub const ENERGY_INVITE: &str = "invite";
pub const ENERGY_BINDING: &str = "binding";

pub const ENERGY_REGISTER_VALUE: i32 = 21000000;
pub const ENERGY_PROPOSAL_VALUE: i32 = -1;
pub const ENERGY_VOTE_VALUE: i32 = -1;
pub const ENERGY_INVITE_VALUE: i32 = -1;
pub const ENERGY_BINDING_VALUE: i32 = -1;

pub const PROPOSAL_STATUS_PENDING: &str = "Pending";
pub const PROPOSAL_STATUS_ACTIVE: &str = "Active";
pub const PROPOSAL_STATUS_PASSED: &str = "Passed";

pub const PROPOSAL_DESCRIPTION_MAX_LENGTH: i32 = 4000;

pub const EVENT_TOPIC: &str = "events";
pub const NOSTR_TOPIC: &str = "nostr";

pub const EVENT_TYPE_VOTE: &str = "vote";
pub const EVENT_TYPE_PROPOSAL: &str = "proposal";
pub const EVENT_TYPE_INVITE: &str = "invite";
pub const EVENT_TYPE_BINDING: &str = "binding";
pub const EVENT_TYPE_REGISTER: &str = "register";
pub const EVENT_TYPE_JOIN: &str = "join";
