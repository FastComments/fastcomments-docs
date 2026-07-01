Comentadores anteriores na página que NÃO estão online no momento. Ordenados por displayName.  
Use isso após consumir /users/online para renderizar uma seção “Members”.  
Paginação por cursor em commenterName: o servidor percorre o parcial {tenantId, urlId, commenterName} índice a partir de afterName avançando via $gt, sem custo de $skip.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| after_name | String | Não |  |
| after_user_id | String | Não |  |

## Response

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'Exemplo get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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