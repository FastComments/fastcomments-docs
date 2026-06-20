### Използване на публичния API

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // Създаване на конфигурация за API
    let config = Configuration::new();

    // Вземане на коментари за страница
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

### Използване на удостоверения API

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // Създаване на конфигурация с API ключ
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Извличане на коментари с удостоверения API
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

### Използване на модерационния API

Методите за модерация захранват таблото на модератора. Те използват API-ключ `Configuration`, точно както удостоверения API, и всеки метод приема незадължителен `sso` токен, така че повикването може да се направи от името на модератор, удостоверен чрез SSO.

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::moderation_api;

#[tokio::main]
async fn main() {
    // Създаване на конфигурация с API ключ
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Броене на коментари, чакащи в опашката за модерация
    let result = moderation_api::get_count(
        &config,
        moderation_api::GetCountParams {
            text_search: None,
            by_ip_from_comment: None,
            filter: None,
            search_filters: None,
            demo: None,
            sso: None, // подайте SSO токен за да действате като модератор, удостоверен чрез SSO
        },
    )
    .await;

    match result {
        Ok(response) => println!("Comments to moderate: {}", response.count),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Използване на SSO за удостоверяване

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // Създаване на сигурни SSO данни за потребителя (само на сървърната страна!)
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // Идентификатор на потребителя
        "user@example.com".to_string(),   // Имейл
        "John Doe".to_string(),            // Потребителско име
        "https://example.com/avatar.jpg".to_string(), // URL на аватара
    );

    // Генериране на SSO токен
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // Предайте този токен на своя фронтенд за удостоверяване
}
```