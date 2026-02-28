//! Shared KV request/response types for OpenRaft examples, using `rkyv`.

use std::fmt;

/// A request to the KV store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum Request {
    Set { key: String, value: String },
}

impl Request {
    pub fn set(key: impl Into<String>, value: impl Into<String>) -> Self {
        Request::Set {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::Set { key, value } => write!(f, "Set {{ key: {}, value: {} }}", key, value),
        }
    }
}

/// A response from the KV store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Response {
    pub value: Option<String>,
}

impl Response {
    pub fn new(value: impl Into<String>) -> Self {
        Response {
            value: Some(value.into()),
        }
    }

    pub fn none() -> Self {
        Response { value: None }
    }
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
);

pub fn encode_request(req: &Request) -> Result<rkyv::util::AlignedVec, rkyv::rancor::Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(req)
}

pub fn decode_request(bytes: &[u8]) -> Result<Request, rkyv::rancor::Error> {
    rkyv::from_bytes::<Request, rkyv::rancor::Error>(bytes)
}

pub fn encode_response(resp: &Response) -> Result<rkyv::util::AlignedVec, rkyv::rancor::Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(resp)
}

pub fn decode_response(bytes: &[u8]) -> Result<Response, rkyv::rancor::Error> {
    rkyv::from_bytes::<Response, rkyv::rancor::Error>(bytes)
}

#[cfg(test)]
mod tests {
    use openraft::raft::VoteRequest;
    use openraft::Vote;

    use super::*;

    #[test]
    fn test_request_rkyv_roundtrip() {
        let req = Request::set("k1", "v1");

        let bytes = encode_request(&req).unwrap();
        let decoded = decode_request(&bytes).unwrap();

        assert_eq!(req, decoded);
    }

    #[test]
    fn test_response_rkyv_roundtrip() {
        let resp = Response::new("v1");

        let bytes = encode_response(&resp).unwrap();
        let decoded = decode_response(&bytes).unwrap();

        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_openraft_vote_request_rkyv_roundtrip() {
        let req = VoteRequest::<TypeConfig>::new(Vote::<TypeConfig>::new(3, 5), None);

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&req).unwrap();
        let decoded = rkyv::from_bytes::<VoteRequest<TypeConfig>, rkyv::rancor::Error>(&bytes).unwrap();

        let archive = rkyv::access::<rkyv::Archived<VoteRequest<TypeConfig>>, rkyv::rancor::Error>(&bytes).unwrap();

        assert_eq!(req, decoded);
        assert!(!archive.vote.committed);
        assert!(archive.last_log_id.is_none());
    }
}
