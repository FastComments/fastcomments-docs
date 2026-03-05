Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | No |  |
| parentTenantId | string | No |  |
| includeStats | bool | No |  |

## Response

Returns: [`Option[AggregationResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregation_response.nim)

## Example

[inline-code-attrs-start title = 'aggregate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.aggregate(
  tenantId = "my-tenant-123",
  aggregationRequest = AggregationRequest(
    groupBy = @["urlPath"],
    filters = @[],
    size = 50
  ),
  parentTenantId = "parent-tenant-456",
  includeStats = true
)

if response.isSome:
  let aggregation = response.get()
  echo "HTTP status: ", $httpResponse.status
  echo "Aggregation result: ", $aggregation
[inline-code-end]
