## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| updateModeratorBody | UpdateModeratorBody | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад updateModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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