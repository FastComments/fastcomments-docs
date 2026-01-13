req
tenantId
urlId
userIdWS

## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| user_id_ws | String | Sim |  |
| start_time | i64 | Sim |  |
| end_time | i64 | Sim |  |

## Resposta

Retorna: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)