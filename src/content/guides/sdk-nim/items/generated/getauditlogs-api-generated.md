## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| limit | float64 | No |  |
| skip | float64 | No |  |
| order | SORTDIR | No |  |
| after | float64 | No |  |
| before | float64 | No |  |

## Response

Returns: [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## Example

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getAuditLogs(tenantId = "my-tenant-123",
  limit = 100.0,
  skip = 0.0,
  order = SORTDIR.DESC,
  after = 0.0,
  before = 0.0)

if response.isSome:
  let logs = response.get()
  discard logs
[inline-code-end]
