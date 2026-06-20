## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 回應

回傳：[`Option[GetTenantPackagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_packages_response.nim)

## 範例

[inline-code-attrs-start title = 'getTenantPackages 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackages(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let packages = response.get()
  echo "Received tenant packages:"
  echo packages
else:
  echo "No packages found for tenant 'my-tenant-123'"
[inline-code-end]

---