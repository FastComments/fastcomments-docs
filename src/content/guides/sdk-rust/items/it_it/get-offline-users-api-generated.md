Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usare questo dopo aver esaurito /users/online per renderizzare una sezione "Members".
Paginazione tramite cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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