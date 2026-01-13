req
tenantId
urlId
userIdWS

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| user_id_ws | String | Sì |  |
| start_time | i64 | Sì |  |
| end_time | i64 | Sì |  |

## Risposta

Restituisce: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)