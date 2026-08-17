Агрегира документи чрез групиране (ако е предоставен groupBy) и прилагане на множество операции. Поддържат се различни операции (например sum, countDistinct, avg и др.).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Да |  |
| parentTenantId | string | Не |  |
| includeStats | boolean | Не |  |

## Отговор

Връща: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Пример

[inline-code-attrs-start title = 'пример за агрегиране'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-12345";

const aggregationRequest: AggregationRequest = {
  predicates: [
    {
      field: "status",
      operator: "eq",
      value: { type: "string", value: "approved" }
    }
  ],
  operations: [
    { type: "count", field: "commentId" }
  ],
  sort: { field: "createdAt", direction: "desc" }
};

const parentTenantId: string = "parent-001";
const includeStats: boolean = true;

const result: AggregateResponse = await aggregate(
  tenantId,
  aggregationRequest,
  parentTenantId,
  includeStats
);
[inline-code-end]

---