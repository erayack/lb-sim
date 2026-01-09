use thiserror::Error;

const ERR_EMPTY_SERVERS: &str = "servers must not be empty";
const ERR_EMPTY_SERVER_ENTRY: &str = "servers must not contain empty entries";
const ERR_REQUESTS_ZERO: &str = "requests must be greater than 0";
const ERR_DUPLICATE_SERVER_NAME: &str = "duplicate server name";
const ERR_INVALID_SERVER_ENTRY: &str = "invalid server entry";
const ERR_INVALID_LATENCY: &str = "invalid latency in";
const ERR_INVALID_LATENCY_VALUE: &str = "latency must be > 0 in";
const ERR_INVALID_WEIGHT: &str = "invalid weight in";
const ERR_INVALID_WEIGHT_VALUE: &str = "weight must be > 0 in";
const ERR_INVALID_REQUEST_RATE: &str = "request rate must be > 0";
const ERR_INVALID_REQUEST_DURATION: &str = "request duration must be > 0";
const ERR_INVALID_TIE_BREAK_SEED: &str = "tie-break seed required when tie_break is seeded";
const ERR_UNSUPPORTED_CONFIG_FORMAT: &str = "unsupported config format";

#[derive(Error, Debug)]
pub enum Error {
    #[error("{ERR_EMPTY_SERVERS}")]
    EmptyServers,
    #[error("{ERR_EMPTY_SERVER_ENTRY}")]
    EmptyServerEntry,
    #[error("{ERR_REQUESTS_ZERO}")]
    RequestsZero,
    #[error("{ERR_DUPLICATE_SERVER_NAME} '{0}'")]
    DuplicateServerName(String),
    #[error("{ERR_INVALID_SERVER_ENTRY} '{0}': expected name:latency_ms[:weight]")]
    InvalidServerEntry(String),
    #[error("{ERR_INVALID_LATENCY} '{0}'")]
    InvalidLatency(String),
    #[error("{ERR_INVALID_LATENCY_VALUE} '{0}'")]
    InvalidLatencyValue(String),
    #[error("{ERR_INVALID_WEIGHT} '{0}'")]
    InvalidWeight(String),
    #[error("{ERR_INVALID_WEIGHT_VALUE} '{0}'")]
    InvalidWeightValue(String),
    #[error("{ERR_INVALID_REQUEST_RATE} (got {0})")]
    InvalidRequestRate(f64),
    #[error("{ERR_INVALID_REQUEST_DURATION} (got {0}ms)")]
    InvalidRequestDuration(u64),
    #[error("{ERR_INVALID_TIE_BREAK_SEED}")]
    InvalidTieBreakSeed,
    #[error("{0}")]
    ConfigIo(String),
    #[error("{0}")]
    ConfigParse(String),
    #[error("{ERR_UNSUPPORTED_CONFIG_FORMAT} '{0}'")]
    UnsupportedConfigFormat(String),
    #[error("{0}")]
    Cli(String),
}

pub type Result<T> = std::result::Result<T, Error>;
