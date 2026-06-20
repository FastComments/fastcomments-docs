Tidligere kommentatorer på siden, som ikke er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at vise en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren gennemgår det delvise indeks {tenantId, urlId, commenterName} fra afterName fremad via $gt, uden omkostning ved $skip.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nej |  |
| after_user_id | String | Nej |  |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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