Агрегира документи чрез групиране (ако е предоставен groupBy) и прилагане на множество операции.
Поддържат се различни операции (напр. sum, countDistinct, avg и т.н.).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| aggregation_request | models::AggregationRequest | Да |  |
| parent_tenant_id | String | Не |  |
| include_stats | bool | Не |  |

## Отговор

Връща: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---