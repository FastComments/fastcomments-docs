## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| badge_id | String | Sì |  |
| user_id | String | No |  |
| comment_id | String | No |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/remove_user_badge_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio put_remove_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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