Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página neste momento.  
Retorna anonCount + totalCount (assinantes de toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| after_name | String | Não |  |
| after_user_id | String | Não |  |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]

---