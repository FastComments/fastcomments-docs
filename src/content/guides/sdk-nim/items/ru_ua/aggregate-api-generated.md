## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Нет |  |
| parentTenantId | string | Нет |  |
| includeStats | bool | Нет |  |

## Response

Возвращает: [`Option[AggregationResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregation_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(),
  parentTenantId = "",
  includeStats = false
)
if response.isSome:
  let aggregation = response.get()
  echo $aggregation
[inline-code-end]