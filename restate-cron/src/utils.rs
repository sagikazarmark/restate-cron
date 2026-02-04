use restate_sdk::prelude::Request;

pub trait RequestExt<'a, Req, Res = ()> {
    fn idempotency_key_maybe(self, key: Option<String>) -> Request<'a, Req, Res>;
}

impl<'a, Req, Res> RequestExt<'a, Req, Res> for Request<'a, Req, Res> {
    fn idempotency_key_maybe(self, key: Option<String>) -> Request<'a, Req, Res> {
        if let Some(key) = key {
            return self.idempotency_key(key);
        }

        self
    }
}
