/// `RetryOptions`, without its `shouldRetry` closure, which takes an
/// AsyncHTTPClient response. Without that closure, a client retries only a
/// request that fails outright, never one that a server error answers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RetryOptions {
  pub max_retries: isize,
  /// In nanoseconds.
  pub retry_interval: u64,
}

impl RetryOptions {
  /// `RetryOptions(maxRetries:retryInterval:)`.
  pub fn new(max_retries: isize, retry_interval: u64) -> Self {
    Self {
      max_retries,
      retry_interval,
    }
  }
}
