Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Membros".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
índice a partir de afterName em diante via $gt, sem custo de $skip.

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| after_name | String | Não |  |
| after_user_id | String | Não |  |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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