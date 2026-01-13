## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| after_id | String | Não |  |
| after_created_at | i64 | Não |  |
| unread_only | bool | Não |  |
| dm_only | bool | Não |  |
| no_dm | bool | Não |  |
| sso | String | Não |  |

## Resposta

Retorna: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_200_response.rs)

---