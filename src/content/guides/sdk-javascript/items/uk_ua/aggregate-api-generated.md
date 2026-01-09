Агрегує документи, групуючи їх (якщо вказано groupBy) та застосовуючи кілька операцій.
Підтримуються різні операції (наприклад sum, countDistinct, avg тощо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| aggregationRequest | AggregationRequest | Так |  |
| parentTenantId | string | Ні |  |
| includeStats | boolean | Ні |  |

## Відповідь

Повертає: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)