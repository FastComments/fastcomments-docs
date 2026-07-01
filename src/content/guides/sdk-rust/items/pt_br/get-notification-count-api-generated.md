## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |
| url_id | String | Não |  |
| from_comment_id | String | Não |  |
| viewed | bool | Não |  |

## Resposta

Retorna: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## Exemplo

[inline-code-attrs-start title = 'get_notification_count Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("john.doe".to_string()),
        url_id: Some("blog/post-123".to_string()),
        from_comment_id: Some("comment789".to_string()),
        viewed: Some(true),
    };
    let _response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]