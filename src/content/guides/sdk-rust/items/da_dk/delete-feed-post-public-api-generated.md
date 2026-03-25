## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| post_id | String | Ja |  |
| broadcast_id | String | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_feed_post_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---