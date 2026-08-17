Агрегира документе груписањем (ако је наведен groupBy) и примењивањем више операција.  
Подржане су различите операције (нпр. sum, countDistinct, avg, итд.).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| aggregationRequest | AggregationRequest | Не |  |
| options | AggregateOptions | Не |  |

## Одговор

Returns: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Пример

[inline-code-attrs-start title = 'пример агрегирања'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]