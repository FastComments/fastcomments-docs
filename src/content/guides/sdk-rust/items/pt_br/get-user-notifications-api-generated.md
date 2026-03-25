## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| page_size | i32 | Não |  |
| after_id | String | Não |  |
| include_context | bool | Não |  |
| after_created_at | i64 | Não |  |
| unread_only | bool | Não |  |
| dm_only | bool | Não |  |
| no_dm | bool | Não |  |
| include_translations | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUserNotificationsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    page_size: Some(25),
    after_id: Some("notif_98765".to_string()),
    include_context: Some(true),
    after_created_at: Some(1_681_500_000i64),
    unread_only: Some(true),
    dm_only: Some(false),
    no_dm: Some(false),
    include_translations: Some(true),
    sso: Some("sso_user_token_ab12".to_string()),
};
let notifications: GetUserNotifications200Response = get_user_notifications(&configuration, params).await?;
[inline-code-end]

---