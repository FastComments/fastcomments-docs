## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| limit | f64 | Não |  |
| skip | f64 | Não |  |
| order | models::SortDir | Não |  |
| after | f64 | Não |  |
| before | f64 | Não |  |

## Resposta

Retorna: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---