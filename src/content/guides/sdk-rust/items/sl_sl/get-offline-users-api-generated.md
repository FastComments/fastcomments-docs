---
Pretekli komentatorji na strani, ki trenutno NISO na spletu. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online, da prikažete razdelek "Člani".
Paginacija s kurzorjem po commenterName: strežnik prehaja delni {tenantId, urlId, commenterName}
indeks od afterName naprej preko $gt, brez stroška $skip.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| after_name | String | Ne |  |
| after_user_id | String | Ne |  |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Primer

[inline-code-attrs-start title = 'get_offline_users Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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