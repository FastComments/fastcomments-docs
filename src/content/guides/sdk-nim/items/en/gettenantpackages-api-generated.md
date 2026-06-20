## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetTenantPackagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages_response.nim)

## Example

[inline-code-attrs-start title = 'getTenantPackages Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let packages = response.get()
  echo "Received tenant packages:"
  echo packages
else:
  echo "No packages found for tenant 'my-tenant-123'"
[inline-code-end]
