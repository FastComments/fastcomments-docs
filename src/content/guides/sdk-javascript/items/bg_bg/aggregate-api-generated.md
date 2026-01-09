Агрегира документи чрез групиране (ако е зададен groupBy) и прилагане на множество операции. Поддържат се различни операции (например sum, countDistinct, avg и др.).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Да |  |
| parentTenantId | string | Не |  |
| includeStats | boolean | Не |  |

## Отговор

Връща: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---