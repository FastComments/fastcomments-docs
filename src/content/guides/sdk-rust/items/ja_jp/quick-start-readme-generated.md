### パブリック API の使用

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // API 設定を作成
    let config = Configuration::new();

    // ページのコメントを取得
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

### 認証済み API の使用

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // API キーで設定を作成
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // 認証済み API を使ってコメントを取得
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

### モデレーション API の使用

モデレーション用のメソッドはモデレーターダッシュボードのバックエンドで動作します。これらは認証済み API と同様に API キーの `Configuration` を使用し、各メソッドはオプションの `sso` トークンを受け付けるため、SSO 認証されたモデレータを代理して呼び出すことができます。

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::moderation_api;

#[tokio::main]
async fn main() {
    // API キーで設定を作成
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // モデレーションキューで待機中のコメントをカウント
    let result = moderation_api::get_count(
        &config,
        moderation_api::GetCountParams {
            text_search: None,
            by_ip_from_comment: None,
            filter: None,
            search_filters: None,
            demo: None,
            sso: None, // SSO 認証されたモデレータとして動作するために SSO トークンを渡す
        },
    )
    .await;

    match result {
        Ok(response) => println!("Comments to moderate: {}", response.count),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### SSO を使用した認証

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // セキュアな SSO ユーザーデータを作成（サーバーサイドのみ！）
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // ユーザー ID
        "user@example.com".to_string(),   // メールアドレス
        "John Doe".to_string(),            // ユーザー名
        "https://example.com/avatar.jpg".to_string(), // アバター URL
    );

    // SSO トークンを生成
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // このトークンをフロントエンドに渡して認証に使用する
}
```