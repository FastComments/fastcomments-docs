### पब्लिक API का उपयोग करना

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // API कॉन्फ़िगरेशन बनाएँ
    let config = Configuration::new();

    // किसी पेज के लिए कमेंट्स प्राप्त करें
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

### ऑथेंटिकेटेड API का उपयोग करना

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // API की के साथ कॉन्फ़िगरेशन बनाएँ
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // ऑथेंटिकेटेड API का उपयोग करके कमेंट्स प्राप्त करें
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

### मॉडरेशन API का उपयोग करना

मॉडरेशन मेथड्स मॉडरेटर डैशबोर्ड को सपोर्ट करते हैं। ये ऑथेंटिकेटेड API की तरह ही API-की वाले `Configuration` का उपयोग करते हैं, और प्रत्येक मेथड एक वैकल्पिक `sso` टोकन स्वीकार करता है ताकि कॉल को SSO-ऑथेंटिकेटेड मॉडरेटर की ओर से किया जा सके।

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::moderation_api;

#[tokio::main]
async fn main() {
    // API की के साथ कॉन्फ़िगरेशन बनाएँ
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // मॉडरेशन क्यू में प्रतीक्षारत कमेंट्स की गिनती करें
    let result = moderation_api::get_count(
        &config,
        moderation_api::GetCountParams {
            text_search: None,
            by_ip_from_comment: None,
            filter: None,
            search_filters: None,
            demo: None,
            sso: None, // SSO-ऑथेंटिकेटेड मॉडरेटर के रूप में कार्य करने के लिए SSO टोकन पास करें
        },
    )
    .await;

    match result {
        Ok(response) => println!("Comments to moderate: {}", response.count),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### ऑथेंटिकेशन के लिए SSO का उपयोग करना

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // सुरक्षित SSO यूज़र डेटा बनाएँ (केवल सर्वर-साइड!)
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // यूज़र ID
        "user@example.com".to_string(),   // ईमेल
        "John Doe".to_string(),            // यूज़रनेम
        "https://example.com/avatar.jpg".to_string(), // अवतार URL
    );

    // SSO टोकन जनरेट करें
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // ऑथेंटिकेशन के लिए यह टोकन अपने फ्रंटएंड को पास करें
}
```