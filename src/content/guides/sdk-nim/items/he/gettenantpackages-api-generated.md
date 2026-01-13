## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | float64 | לא |  |

## תגובה

מחזיר: [`Option[GetTenantPackages_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantPackages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let packages = response.get()
  echo "Received packages for tenant:", " my-tenant-123"
  echo packages
else:
  echo "No packages found, status:", httpResponse.status
[inline-code-end]

---