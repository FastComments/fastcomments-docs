## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateTenantUserBody | UpdateTenantUserBody | 否 |  |
| updateComments | string = "" | 否 |  |

## 回應

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'updateTenantUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantUserBody()
let (optResp, httpResp) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateTenantUserBody = updateBody,
  updateComments = "Changed role to moderator",
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]

---