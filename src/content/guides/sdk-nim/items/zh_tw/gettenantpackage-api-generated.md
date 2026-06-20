## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

回傳: [`Option[GetTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package_response.nim)

## 範例

[inline-code-attrs-start title = 'getTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "premium-2026")
if response.isSome:
  let pkg = response.get()
  echo "Retrieved tenant package:"
  echo pkg
else:
  echo "Tenant package not found"
[inline-code-end]

---