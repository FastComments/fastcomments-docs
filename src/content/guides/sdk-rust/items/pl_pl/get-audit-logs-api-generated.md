## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| limit | f64 | Nie |  |
| skip | f64 | Nie |  |
| order | models::SortDir | Nie |  |
| after | f64 | Nie |  |
| before | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---