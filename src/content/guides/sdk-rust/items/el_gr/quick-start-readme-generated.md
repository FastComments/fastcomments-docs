### Χρήση του δημόσιου API

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // Δημιουργία ρύθμισης API
    let config = Configuration::new();

    // Λήψη σχολίων για μια σελίδα
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

### Χρήση του API με έλεγχο ταυτότητας

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // Δημιουργία διαμόρφωσης με κλειδί API
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Λήψη σχολίων χρησιμοποιώντας το API με έλεγχο ταυτότητας
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

### Χρήση του API Μετριοποίησης

Οι μέθοδοι μετριοποίησης υποστηρίζουν τον πίνακα ελέγχου του διαχειριστή. Χρησιμοποιούν μια `Configuration` με κλειδί API όπως και το API με έλεγχο ταυτότητας, και κάθε μέθοδος δέχεται ένα προαιρετικό token `sso` ώστε η κλήση να μπορεί να γίνει εκ μέρους ενός διαχειριστή που έχει πιστοποιηθεί μέσω SSO.

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::moderation_api;

#[tokio::main]
async fn main() {
    // Δημιουργία διαμόρφωσης με κλειδί API
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Μέτρηση σχολίων που περιμένουν στην ουρά μετριοποίησης
    let result = moderation_api::get_count(
        &config,
        moderation_api::GetCountParams {
            text_search: None,
            by_ip_from_comment: None,
            filter: None,
            search_filters: None,
            demo: None,
            sso: None, // περάστε ένα token SSO για να δράσετε ως διαχειριστής πιστοποιημένος μέσω SSO
        },
    )
    .await;

    match result {
        Ok(response) => println!("Comments to moderate: {}", response.count),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Χρήση του SSO για πιστοποίηση

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // Δημιουργία ασφαλών δεδομένων χρήστη SSO (μόνο στο server!)
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // Αναγνωριστικό χρήστη
        "user@example.com".to_string(),   // Email
        "John Doe".to_string(),            // Όνομα χρήστη
        "https://example.com/avatar.jpg".to_string(), // URL avatar
    );

    // Δημιουργία token SSO
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // Περνάτε αυτό το token στο frontend σας για πιστοποίηση
}
```