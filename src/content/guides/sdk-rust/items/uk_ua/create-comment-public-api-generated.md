## Параметри

| Name | Type | Обов'язкове | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| broadcast_id | String | Так |  |
| comment_data | models::CommentData | Так |  |
| session_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/save_comments_response_with_presence.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад create_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn post_public_comment(configuration: &configuration::Configuration) -> Result<SaveCommentsResponseWithPresence, Error> {
    let params: CreateCommentPublicParams = CreateCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/economic-update-2026".to_string(),
        broadcast_id: "broadcast-2026-06-19-001".to_string(),
        comment_data: models::CommentData {
            content: "Great analysis — this clarified a lot of the market dynamics.".to_string(),
            ..Default::default()
        },
        session_id: Some("sess-9f8e7d6c".to_string()),
        sso: Some("sso-jwt-eyJhbGciOi...".to_string()),
    };
    let response: SaveCommentsResponseWithPresence = create_comment_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---