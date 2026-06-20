## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| post_id | String | Да |  |
| broadcast_id | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример delete_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<DeleteFeedPostPublicResponse, Error> {
    let params: DeleteFeedPostPublicParams = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-06-19".to_string(),
        broadcast_id: Some("broadcast-789".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: DeleteFeedPostPublicResponse = delete_feed_post_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---