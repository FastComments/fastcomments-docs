## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| updateModeratorBody | UpdateModeratorBody | 아니요 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'updateModerator 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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