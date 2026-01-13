## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 否 |  |

## 回應

回傳: [`Option[CreateTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package200response.nim)

## 範例

[inline-code-attrs-start title = 'createTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let packageBody = CreateTenantPackageBody(
  packageName = "premium-comments",
  description = "Premium moderation package for news site",
  planId = "plan-pro-2024",
  seats = 100,
  enabled = true,
  features = @["moderation", "analytics", "sentiment"]
)

let (response, httpResponse) = client.createTenantPackage(tenantId = "my-tenant-123", createTenantPackageBody = packageBody)

if response.isSome:
  let pkg = response.get()
  echo "Created package ID: ", pkg.packageId
  echo "Package name: ", pkg.packageName
else:
  echo "Failed to create package, HTTP status: ", httpResponse.status.code
[inline-code-end]

---