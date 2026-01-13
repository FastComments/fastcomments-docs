---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| limit | f64 | Нет |  |
| skip | f64 | Нет |  |
| order | models::SortDir | Нет |  |
| after | f64 | Нет |  |
| before | f64 | Нет |  |

## Ответ

Возвращает: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---