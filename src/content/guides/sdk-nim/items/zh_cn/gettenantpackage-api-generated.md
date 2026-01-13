## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回: [`Option[GetTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package200response.nim)

## 示例

[inline-code-attrs-start title = 'getTenantPackage 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium-001")
if response.isSome:
  let pkg = response.get()
  echo pkg
else:
  echo "No package found for tenant"
[inline-code-end]