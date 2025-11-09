use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Required claims
    pub iss: String,
    #[serde(deserialize_with = "deserialize_aud")]
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub role: String,
    pub aal: String,
    pub session_id: String,
    pub email: String,
    pub phone: String,
    pub is_anonymous: bool,

    // Optional claims
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amr: Option<Vec<AmrEntry>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub project_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmrEntry {
    pub method: String,
    pub timestamp: usize,
}

// Custom deserializer for aud which can be string or array
fn deserialize_aud<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) => Ok(s),
        StringOrVec::Vec(v) => Ok(v.into_iter().next().unwrap_or_default()),
    }
}

impl Claims {
    pub fn user_id(&self) -> &str {
        &self.sub
    }
}

#[derive(Deserialize)]
struct JwksResponse {
    keys: Vec<JwkResponse>,
}

#[derive(Deserialize)]
struct JwkResponse {
    kid: String,
    n: String,
    e: String,
    #[allow(dead_code)]
    kty: Option<String>,
    #[allow(dead_code)]
    alg: Option<String>,
}

async fn fetch_and_cache_jwks(
    jwks_url: &str,
    anon_key: &str,
    cache: &dashmap::DashMap<String, crate::Jwk>,
) -> Result<(), String> {
    tracing::info!("Fetching JWKS from: {}", jwks_url);

    let client = reqwest::Client::builder()
        .user_agent("Pie-Server/1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(jwks_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

    let status = response.status();
    tracing::info!("JWKS response status: {}", status);

    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read error response".to_string());
        return Err(format!(
            "JWKS request failed with status: {} - Response: {}",
            status, error_text
        ));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read JWKS response: {}", e))?;

    tracing::info!("JWKS response: {}", response_text);

    let jwks: JwksResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JWKS: {} - Response: {}", e, response_text))?;

    tracing::info!("Found {} keys in JWKS response", jwks.keys.len());

    cache.clear();
    for jwk in jwks.keys {
        tracing::info!("Caching JWK with kid: {}", jwk.kid);
        cache.insert(
            jwk.kid.clone(),
            crate::Jwk {
                kid: jwk.kid,
                n: jwk.n,
                e: jwk.e,
            },
        );
    }

    tracing::info!("Cached {} JWKs", cache.len());
    Ok(())
}

async fn get_jwk(
    jwks_url: &str,
    anon_key: &str,
    kid: &str,
    cache: &dashmap::DashMap<String, crate::Jwk>,
) -> Result<crate::Jwk, String> {
    // Check cache first
    if let Some(jwk) = cache.get(kid) {
        return Ok(jwk.clone());
    }

    // Cache miss - fetch and cache all keys
    fetch_and_cache_jwks(jwks_url, anon_key, cache).await?;

    // Try cache again
    cache
        .get(kid)
        .map(|jwk| jwk.clone())
        .ok_or_else(|| format!("Key with kid {} not found in JWKS", kid))
}

pub async fn validate_jwt_token(token: &str, state: &AppState) -> Result<Claims, String> {
    // Decode header to get the algorithm
    let header =
        decode_header(token).map_err(|e| format!("Failed to decode header: {}", e))?;

    tracing::info!("JWT algorithm: {:?}", header.alg);

    // Determine which validation method to use based on algorithm
    match header.alg {
        Algorithm::HS256 => {
            // Use HMAC with JWT secret
            let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes());

            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_exp = true;
            validation.leeway = 60;
            // Supabase uses "authenticated" as the audience for user tokens
            validation.set_audience(&["authenticated"]);

            let token_data = decode::<Claims>(token, &decoding_key, &validation)
                .map_err(|e| format!("JWT validation failed (HS256): {}", e))?;

            Ok(token_data.claims)
        }
        Algorithm::RS256 => {
            // Use RSA with JWKS
            let kid = header
                .kid
                .ok_or("Missing kid in token header for RS256")?;

            let jwk = get_jwk(&state.jwks_url, &state.supabase_anon_key, &kid, &state.jwks_cache)
                .await?;

            let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                .map_err(|e| format!("Failed to create decoding key: {}", e))?;

            let mut validation = Validation::new(Algorithm::RS256);
            validation.validate_exp = true;
            validation.leeway = 60;
            // Supabase uses "authenticated" as the audience for user tokens
            validation.set_audience(&["authenticated"]);

            let token_data = decode::<Claims>(token, &decoding_key, &validation)
                .map_err(|e| format!("JWT validation failed (RS256): {}", e))?;

            Ok(token_data.claims)
        }
        alg => Err(format!("Unsupported algorithm: {:?}", alg)),
    }
}

