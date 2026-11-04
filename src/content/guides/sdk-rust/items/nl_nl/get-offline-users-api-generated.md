Eerdere reageerders op de pagina die momenteel NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat je /users/online hebt uitgeput om een "Leden"-sectie weer te geven.
Cursorpaginering op commenterName: de server loopt via $gt door de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit, zonder $skip-kosten.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nee |  |
| after_user_id | String | Nee |  |

## Response

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]