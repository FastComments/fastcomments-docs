## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| post_id | String | Ja |  |
| broadcast_id | String | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_feed_post_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-123".to_string(),
        broadcast_id: Some("broadcast-456".to_string()),
        sso: Some("sso-token-789".to_string()),
    };
    let _response: DeleteFeedPostPublicResponse = delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]