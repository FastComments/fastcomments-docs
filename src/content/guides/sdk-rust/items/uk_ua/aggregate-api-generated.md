---
Агрегує документи, групуючи їх (якщо надано groupBy) та застосовуючи кілька операцій. Підтримуються різні операції (наприклад, sum, countDistinct, avg тощо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| aggregation_request | models::AggregationRequest | Так |  |
| parent_tenant_id | String | Ні |  |
| include_stats | bool | Ні |  |

## Відповідь

Повертає: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---