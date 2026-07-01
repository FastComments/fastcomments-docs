## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| after_id | String | Não |  |
| after_created_at | i64 | Não |  |
| unread_only | bool | Não |  |
| dm_only | bool | Não |  |
| no_dm | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## Exemplo

[inline-code-attrs-start title = 'reset_user_notifications Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif-12345".to_string()),
        after_created_at: Some(1_640_995_200),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(true),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ResetUserNotificationsResponse =
        reset_user_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]