## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package200response.nim)

## Example

[inline-code-attrs-start title = 'getTenantPackage Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let tenantPackage = response.get()
  echo "Retrieved tenant package:"
  echo tenantPackage
else:
  echo "No package found, HTTP status: ", httpResponse.status
[inline-code-end]
