---
## 매개변수

| Name | Type | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantPackageBody | CreateTenantPackageBody | 아니오 |  |

## 응답

반환: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## 예제

[inline-code-attrs-start title = 'createTenantPackage 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantPackage(tenantId = "my-tenant-123", createTenantPackageBody = CreateTenantPackageBody())

if response.isSome:
  let pkg = response.get()
  echo "Created tenant package: ", $pkg
else:
  echo "Failed to create tenant package, HTTP response: ", $httpResponse
[inline-code-end]

---