---
문서를 그룹화(groupBy가 제공된 경우)하고 여러 연산을 적용하여 집계합니다.
다양한 연산(예: sum, countDistinct, avg 등)을 지원합니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| parentTenantId | string | No |  |
| includeStats | bool | No |  |

## 응답

반환: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## 예제

[inline-code-attrs-start title = 'aggregate 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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