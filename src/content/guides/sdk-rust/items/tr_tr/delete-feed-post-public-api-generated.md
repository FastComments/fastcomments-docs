## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| post_id | String | Evet |  |
| broadcast_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_feed_post_public Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteFeedPostPublicParams = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-03-25-12345".to_string(),
        broadcast_id: Some("broadcast-9876".to_string()),
        sso: Some("user-42-sso-token".to_string()),
    };
    let response: DeleteFeedPostPublic200Response = delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]