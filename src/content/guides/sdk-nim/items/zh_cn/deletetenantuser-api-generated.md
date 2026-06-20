## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| deleteComments | string | 否 |  |
| commentDeleteMode | string | 否 |  |

## 响应

返回: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'deleteTenantUser 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-789", deleteComments = "true", commentDeleteMode = "soft")
if response.isSome:
  let apiResp = response.get()
  echo "Tenant user deleted, response: ", apiResp
else:
  echo "Failed to delete tenant user, HTTP status: ", $httpResponse.status
[inline-code-end]

---