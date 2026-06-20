מאגד מסמכים על ידי קיבוצם (אם מסופק groupBy) והחלת מספר פעולות.
נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו').

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| aggregationRequest | AggregationRequest | לא |  |
| parentTenantId | string | לא |  |
| includeStats | bool | לא |  |

## תגובה

מחזיר: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-aggregate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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