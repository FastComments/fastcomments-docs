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
let (response, httpResponse) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), parentTenantId = "", includeStats = false)
if response.isSome:
  let agg = response.get()
  echo "Aggregation succeeded: ", $agg
else:
  echo "Aggregation failed, HTTP response: ", $httpResponse
[inline-code-end]
