## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | No |  |

## 응답

반환: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## 예시

[inline-code-attrs-start title = 'createTenantPackage 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody()
)

if responseOpt.isSome:
  let response = responseOpt.get()
[inline-code-end]