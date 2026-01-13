## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |
| url_id | String | Não |  |
| from_comment_id | String | Não |  |
| viewed | bool | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-1234".to_string()),
        url_id: Some("news/politics/article-2026-01-12".to_string()),
        from_comment_id: Some("cmt-98765".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotifications200Response = get_notifications(&configuration, params).await?;
    let _ = notifications;
    Ok(())
}
[inline-code-end]

---