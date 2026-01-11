---
req
tenantId
urlId
userIdWS

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| user_id_ws | String | Tak |  |
| start_time | i64 | Tak |  |
| end_time | i64 | Tak |  |

## Odpowiedź

Zwraca: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_200_response.rs)

---