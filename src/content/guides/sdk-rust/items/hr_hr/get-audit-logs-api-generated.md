---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| limit | f64 | Ne |  |
| skip | f64 | Ne |  |
| order | models::SortDir | Ne |  |
| after | f64 | Ne |  |
| before | f64 | Ne |  |

## Odgovor

Vraća: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---