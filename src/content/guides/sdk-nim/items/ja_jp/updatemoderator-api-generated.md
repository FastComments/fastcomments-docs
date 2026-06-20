---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateModeratorBody | UpdateModeratorBody | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'updateModerator の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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