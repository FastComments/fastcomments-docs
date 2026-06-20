Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usalo dopo aver esaurito /users/online per visualizzare una sezione "Membri".
Paginazione cursore su commenterName: il server scorre l'indice parziale {tenantId, urlId, commenterName} da afterName in avanti tramite $gt, senza costo di $skip.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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