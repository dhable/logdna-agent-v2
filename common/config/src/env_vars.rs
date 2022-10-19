pub const INGESTION_KEY: &str = "MZ_INGESTION_KEY";
pub const CONFIG_FILE: &str = "MZ_CONFIG_FILE";
pub const LOG_DIRS: &str = "MZ_LOG_DIRS";
pub const TAGS: &str = "MZ_TAGS";
pub const HOST: &str = "MZ_HOST";
pub const ENDPOINT: &str = "MZ_ENDPOINT";
pub const USE_SSL: &str = "MZ_USE_SSL";
pub const USE_COMPRESSION: &str = "MZ_USE_COMPRESSION";
pub const GZIP_LEVEL: &str = "MZ_GZIP_LEVEL";
pub const EXCLUSION_RULES: &str = "MZ_EXCLUSION_RULES";
pub const EXCLUSION_REGEX_RULES: &str = "MZ_EXCLUSION_REGEX_RULES";
pub const INCLUSION_RULES: &str = "MZ_INCLUSION_RULES";
pub const INCLUSION_REGEX_RULES: &str = "MZ_INCLUSION_REGEX_RULES";
pub const HOSTNAME: &str = "MZ_HOSTNAME";
pub const IP: &str = "MZ_IP";
pub const MAC: &str = "MZ_MAC";
pub const JOURNALD_PATHS: &str = "MZ_JOURNALD_PATHS";
pub const LOOKBACK: &str = "MZ_LOOKBACK";
pub const DB_PATH: &str = "MZ_DB_PATH";
pub const METRICS_PORT: &str = "MZ_METRICS_PORT";
pub const USE_K8S_LOG_ENRICHMENT: &str = "MZ_USE_K8S_LOG_ENRICHMENT";
pub const LOG_K8S_EVENTS: &str = "MZ_LOG_K8S_EVENTS";
pub const LOG_METRIC_SERVER_STATS: &str = "MZ_LOG_METRIC_SERVER_STATS";
pub const K8S_STARTUP_LEASE: &str = "MZ_K8S_STARTUP_LEASE";
pub const LINE_EXCLUSION: &str = "MZ_LINE_EXCLUSION_REGEX";
pub const LINE_INCLUSION: &str = "MZ_LINE_INCLUSION_REGEX";
pub const REDACT: &str = "MZ_REDACT_REGEX";
pub const INGEST_TIMEOUT: &str = "MZ_INGEST_TIMEOUT";
pub const INGEST_BUFFER_SIZE: &str = "MZ_INGEST_BUFFER_SIZE";
pub const RETRY_DIR: &str = "MZ_RETRY_DIR";
pub const RETRY_DISK_LIMIT: &str = "MZ_RETRY_DISK_LIMIT";
pub const INTERNAL_FS_DELAY: &str = "MZ_INTERNAL_FS_DELAY";

// unused or deprecated
pub const INGESTION_KEY_ALTERNATE: &str = "LOGDNA_AGENT_KEY";
pub const CONFIG_FILE_DEPRECATED: &str = "DEFAULT_CONF_FILE";
pub const HOST_DEPRECATED: &str = "LDLOGHOST";
pub const IBM_HOST_DEPRECATED: &str = "LOGDNA_LOGHOST";
pub const ENDPOINT_DEPRECATED: &str = "LDLOGPATH";
pub const USE_SSL_DEPRECATED: &str = "LDLOGSSL";
pub const USE_COMPRESSION_DEPRECATED: &str = "COMPRESS";
pub const GZIP_LEVEL_DEPRECATED: &str = "GZIP_COMPRESS_LEVEL";
pub const LOG_DIRS_DEPRECATED: &str = "LOG_DIRS";
pub const EXCLUSION_RULES_DEPRECATED: &str = "LOGDNA_EXCLUDE";
pub const EXCLUSION_REGEX_RULES_DEPRECATED: &str = "LOGDNA_EXCLUDE_REGEX";
pub const INCLUSION_RULES_DEPRECATED: &str = "LOGDNA_INCLUDE";
pub const INCLUSION_REGEX_RULES_DEPRECATED: &str = "LOGDNA_INCLUDE_REGEX";

pub const META_APP: &str = "MZ_META_APP";
pub const META_HOST: &str = "MZ_META_HOST";
pub const META_ENV: &str = "MZ_META_ENV";
pub const META_FILE: &str = "MZ_META_FILE";
pub const META_K8S_FILE: &str = "MZ_META_K8S_FILE";
pub const META_JSON: &str = "MZ_META_JSON";
pub const META_ANNOTATIONS: &str = "MZ_META_ANNOTATIONS";
pub const META_LABELS: &str = "MZ_META_LABELS";

pub const NO_CAP: &str = "MZ_NO_CAP";