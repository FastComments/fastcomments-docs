## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |

## Одговор

Враћа: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Пример

[inline-code-attrs-start title = 'flag_comment Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: FlagCommentParams = FlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-06-19/comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---