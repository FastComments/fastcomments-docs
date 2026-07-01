## 參數

| 名稱 | 類型 | 必須 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 否 |  |

## 回應

返回：[`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## 範例

[inline-code-attrs-start title = 'createTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody()
)

if responseOpt.isSome:
  let response = responseOpt.get()
[inline-code-end]