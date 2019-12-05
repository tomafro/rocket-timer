use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};

use std::time::SystemTime;
use uuid::Uuid;

/// Fairing for timing requests.
pub struct ServerTiming;

/// Value stored in request-local state.
#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

impl Fairing for ServerTiming {
    fn info(&self) -> Info {
        Info {
            name: "Request Timer",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Stores the start time of the request in request-local state.
    fn on_request(&self, request: &mut Request, _: &Data) {
        // Store a `TimerStart` instead of directly storing a `SystemTime`
        // to ensure that this usage doesn't conflict with anything else
        // that might store a `SystemTime` in request-local cache.
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    /// Adds a header to the response indicating how long the server took to
    /// process the request.
    fn on_response(&self, request: &Request, response: &mut Response) {
        let start_time = request.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            response.set_raw_header("Server-Timing", format!("overall;dur={}ms;desc=\"Overall\"", ms));
        }
    }
}

struct RequestId(Option<Uuid>);

pub struct RequestIdHeader;

impl Fairing for RequestIdHeader {
    fn info(&self) -> Info {
        Info {
            name: "Request Id",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Stores the start time of the request in request-local state.
    fn on_request(&self, request: &mut Request, _: &Data) {
        // Store a `TimerStart` instead of directly storing a `SystemTime`
        // to ensure that this usage doesn't conflict with anything else
        // that might store a `SystemTime` in request-local cache.
        request.local_cache(|| RequestId(Some(Uuid::new_v4())));
    }

    /// Adds a header to the response indicating how long the server took to
    /// process the request.
    fn on_response(&self, request: &Request, response: &mut Response) {
        let request_id = request.local_cache(|| RequestId(None));
        if let Some(uuid) = request_id.0 {
            response.set_raw_header("X-Request-Id", format!("{}", uuid));
        }
    }
}
