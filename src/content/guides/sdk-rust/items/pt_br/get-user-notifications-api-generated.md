## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
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

---