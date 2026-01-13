---
Агрегирует документы, группируя их (если указан groupBy) и применяя несколько операций.
Поддерживаются разные операции (например, sum, countDistinct, avg и т.д.).

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Да |  |
| parentTenantId | string | Нет |  |
| includeStats | boolean | Нет |  |

## Ответ

Возвращает: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---