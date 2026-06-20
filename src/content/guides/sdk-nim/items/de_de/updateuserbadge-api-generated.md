## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Beispiel

[inline-code-attrs-start title = 'updateUserBadge Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateUserBadge(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateUserBadgeParams = UpdateUserBadgeParams()
)

if response.isSome:
  let success = response.get()
  echo "Badge updated successfully"
else:
  echo "Badge update failed"
[inline-code-end]

---