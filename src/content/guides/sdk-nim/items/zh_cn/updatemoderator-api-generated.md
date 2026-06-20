## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateModeratorBody | UpdateModeratorBody | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'updateModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let modBody: UpdateModeratorBody = UpdateModeratorBody(
  displayName = "Alice Moderator",
  email = "alice@newsdaily.com",
  isActive = true,
  permissions = @["delete_comments", "ban_users"]
)

let (response, httpResponse) = client.updateModerator(tenantId = "news-tenant-456", id = "moderator-789", updateModeratorBody = modBody)

if response.isSome:
  let apiEmpty = response.get()
  echo "Moderator updated successfully. HTTP status: ", httpResponse.status
else:
  echo "Failed to update moderator. HTTP status: ", httpResponse.status
[inline-code-end]

---