Ativar ou desativar notificações para um comentário específico.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| notification_id | String | Sim |  |
| opted_in_or_out | String | Sim |  |
| comment_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)