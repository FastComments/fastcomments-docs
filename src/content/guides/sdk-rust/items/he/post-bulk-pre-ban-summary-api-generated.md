## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| bulk_pre_ban_params | models::BulkPreBanParams | כן |  |
| include_by_user_id_and_email | bool | לא |  |
| include_by_ip | bool | לא |  |
| include_by_email_domain | bool | לא |  |
| sso | String | לא |  |

## תגובה

מחזיר: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_pre_ban_summary.rs)

## דוגמה

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PostBulkPreBanSummaryParams = PostBulkPreBanSummaryParams {
    bulk_pre_ban_params: models::BulkPreBanParams {
        entries: vec![
            models::BulkPreBanEntry {
                user_id: Some("user-8472".to_string()),
                email: Some("malicious.signals@fraudmail.com".to_string()),
                ip: Some("198.51.100.23".to_string()),
            },
            models::BulkPreBanEntry {
                user_id: Some("user-9021".to_string()),
                email: Some("bot.account@spamnews.org".to_string()),
                ip: None,
            },
        ],
        reason: Some("coordinated spam campaign".to_string()),
    },
    include_by_user_id_and_email: Some(true),
    include_by_ip: Some(true),
    include_by_email_domain: Some(true),
    sso: Some("acme-corp-sso".to_string()),
};
let summary: BulkPreBanSummary = post_bulk_pre_ban_summary(&configuration, params).await?
[inline-code-end]

---