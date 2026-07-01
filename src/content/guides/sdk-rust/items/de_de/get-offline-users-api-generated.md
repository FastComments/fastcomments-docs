---
Frühere Kommentatoren auf der Seite, die momentan NICHT online sind. Sortiert nach displayName.  
Verwenden Sie dies, nachdem /users/online erschöpft wurde, um einen „Members“-Abschnitt zu rendern.  
Cursor‑Paginierung über commenterName: Der Server durchläuft den Teil {tenantId, urlId, commenterName} Index ab afterName vorwärts via $gt, ohne $skip‑Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Antwort

Rückgabe: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_offline_users Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---