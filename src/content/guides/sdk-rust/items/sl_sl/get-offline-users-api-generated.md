Pretekli komentatorji na strani, ki NISO trenutno online. Razvrščeni po **displayName**.  
Uporabite to po izčrpavanju `/users/online`, da prikažete razdelek “Člani”.  
Kurzorjeva paginacija po **commenterName**: strežnik hodi po delnem `{tenantId, urlId, commenterName}` indeks od **afterName** naprej prek `$gt`, brez stroška `$skip`.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| after_name | String | Ne |  |
| after_user_id | String | Ne |  |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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