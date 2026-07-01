Агрегує документи, групуючи їх (якщо зазначено groupBy) і застосовуючи кілька операцій.  
Різні операції (наприклад, sum, countDistinct, avg тощо) підтримуються.

## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| options | AggregateOptions | No |  |

## Відповідь

Returns: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## Приклад

[inline-code-attrs-start title = 'aggregate Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]