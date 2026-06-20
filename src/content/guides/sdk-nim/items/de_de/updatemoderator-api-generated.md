## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| updateModeratorBody | UpdateModeratorBody | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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