Агрегує документи шляхом групування (якщо вказано groupBy) та застосування кількох операцій.
Підтримуються різні операції (наприклад sum, countDistinct, avg тощо).

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| aggregationRequest | AggregationRequest | Ні |  |
| parentTenantId | string | Ні |  |
| includeStats | bool | Ні |  |

## Відповідь

Повертає: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(groupBy = @["articleId"], metrics = @["commentCount"], filters = @[], limit = 0),
  parentTenantId = "",
  includeStats = false
)

if response.isSome:
  let agg = response.get()
  discard agg
[inline-code-end]

---