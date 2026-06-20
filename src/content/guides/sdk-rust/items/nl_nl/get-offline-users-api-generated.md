---
Vorige commentatoren op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat u /users/online hebt uitgeput om een sectie "Leden" weer te geven.
Cursor-paginering op commenterName: de server doorloopt de gedeeltelijke {tenantId, urlId, commenterName}
index vanaf afterName naar voren via $gt, geen $skip-kosten.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Antwoord

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---