## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| badge_id | String | Ja |  |
| user_id | String | Nee |  |
| comment_id | String | Nee |  |
| broadcast_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'put_remove_badge Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_badge_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PutRemoveBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        badge_id: "news-contributor".to_string(),
        user_id: Some("user-42".to_string()),
        comment_id: Some("comment-12345".to_string()),
        broadcast_id: None,
        sso: Some("sso-key-xyz".to_string()),
    };
    let _response: RemoveUserBadgeResponse = put_remove_badge(configuration, params).await?;
    Ok(())
}
[inline-code-end]