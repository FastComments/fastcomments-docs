## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 否 |  |

## 回應

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'updateTenantPackage 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateTenantPackageBody()
let (optResp, httpResp) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "premium-plan",
  updateTenantPackageBody = body
)
if optResp.isSome:
  let empty = optResp.get()
[inline-code-end]