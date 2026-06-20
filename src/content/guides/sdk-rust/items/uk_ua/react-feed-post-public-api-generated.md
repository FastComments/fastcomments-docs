## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| post_id | String | Так |  |
| react_body_params | models::ReactBodyParams | Так |  |
| is_undo | bool | Ні |  |
| broadcast_id | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад react_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-06-19".to_string(),
        react_body_params: models::ReactBodyParams {
            reaction: "like".to_string(),
            user_id: "user-9876".to_string(),
            metadata: None,
        },
        is_undo: Some(false),
        broadcast_id: Some("broadcast-42".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: ReactFeedPostResponse = react_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---