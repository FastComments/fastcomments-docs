## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetTenants_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenants200response.nim)

## Example

[inline-code-attrs-start title = 'getTenants Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenants(tenantId = "my-tenant-123", meta = "include=stats", skip = 0.0)
if response.isSome:
  let tenants = response.get()
  echo tenants
else:
  echo "No tenants returned"
[inline-code-end]
