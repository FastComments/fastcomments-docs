## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createTenantPackageBody | CreateTenantPackageBody | いいえ |  |

## レスポンス

返却値: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## 例

[inline-code-attrs-start title = 'createTenantPackage の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantPackage(tenantId = "my-tenant-123", createTenantPackageBody = CreateTenantPackageBody())

if response.isSome:
  let pkg = response.get()
  echo "Created tenant package: ", $pkg
else:
  echo "Failed to create tenant package, HTTP response: ", $httpResponse
[inline-code-end]

---