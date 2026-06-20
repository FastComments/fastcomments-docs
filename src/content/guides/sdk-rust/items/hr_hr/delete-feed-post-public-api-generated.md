---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| post_id | String | Da |  |
| broadcast_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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