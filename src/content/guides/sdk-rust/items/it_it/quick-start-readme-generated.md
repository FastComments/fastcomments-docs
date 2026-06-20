### Utilizzo dell'API pubblica

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // Crea la configurazione dell'API
    let config = Configuration::new();

    // Recupera i commenti per una pagina
    let result = public_api::get_comments_public(
        &config,
        public_api::GetCommentsPublicParams {
            tenant_id: "your-tenant-id".to_string(),
            urlid: Some("page-url-id".to_string()),
            url: None,
            count_only: None,
            skip: None,
            limit: None,
            sort_dir: None,
            page: None,
            sso_hash: None,
            simple_sso_hash: None,
            has_no_comment: None,
            has_comment: None,
            comment_id_filter: None,
            child_ids: None,
            start_date_time: None,
            starts_with: None,
        },
    )
    .await;

    match result {
        Ok(response) => {
            println!("Found {} comments", response.comments.len());
            for comment in response.comments {
                println!("Comment: {:?}", comment);
            }
        }
        Err(e) => eprintln!("Error fetching comments: {:?}", e),
    }
}
```

### Utilizzo dell'API autenticata

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // Crea la configurazione con la chiave API
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Recupera i commenti usando l'API autenticata
    let result = default_api::get_comments(
        &config,
        default_api::GetCommentsParams {
            tenant_id: "your-tenant-id".to_string(),
            skip: None,
            limit: None,
            sort_dir: None,
            urlid: Some("page-url-id".to_string()),
            url: None,
            is_spam: None,
            user_id: None,
            all_comments: None,
            for_moderation: None,
            parent_id: None,
            is_flagged: None,
            is_flagged_tag: None,
            is_by_verified: None,
            is_pinned: None,
            asc: None,
            include_imported: None,
            origin: None,
            tags: None,
        },
    )
    .await;

    match result {
        Ok(response) => {
            println!("Total comments: {}", response.count);
            for comment in response.comments {
                println!("Comment ID: {}, Text: {}", comment.id, comment.comment);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Utilizzo dell'API di moderazione

I metodi di moderazione supportano la dashboard dei moderatori. Usano una `Configuration` con chiave API proprio come l'API autenticata, e ciascun metodo accetta un token `sso` opzionale in modo che la chiamata possa essere effettuata per conto di un moderatore autenticato tramite SSO.

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::moderation_api;

#[tokio::main]
async fn main() {
    // Crea la configurazione con la chiave API
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Conta i commenti in attesa nella coda di moderazione
    let result = moderation_api::get_count(
        &config,
        moderation_api::GetCountParams {
            text_search: None,
            by_ip_from_comment: None,
            filter: None,
            search_filters: None,
            demo: None,
            sso: None, // passa un token SSO per agire come moderatore autenticato tramite SSO
        },
    )
    .await;

    match result {
        Ok(response) => println!("Comments to moderate: {}", response.count),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Utilizzo di SSO per l'autenticazione

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // Crea i dati utente SSO sicuri (solo lato server!)
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // ID utente
        "user@example.com".to_string(),   // Email
        "John Doe".to_string(),            // Nome utente
        "https://example.com/avatar.jpg".to_string(), // URL dell'avatar
    );

    // Genera il token SSO
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // Passa questo token al tuo frontend per l'autenticazione
}
```