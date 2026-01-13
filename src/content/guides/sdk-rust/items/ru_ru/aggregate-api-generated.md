---
Агрегирует документы, группируя их (если указан groupBy) и применяя несколько операций.
Поддерживаются различные операции (например sum, countDistinct, avg и т.д.).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| aggregation_request | models::AggregationRequest | Да |  |
| parent_tenant_id | String | Нет |  |
| include_stats | bool | Нет |  |

## Ответ

Возвращает: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---