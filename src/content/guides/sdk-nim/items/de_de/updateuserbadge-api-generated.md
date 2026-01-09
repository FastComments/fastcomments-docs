## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Nein |  |

## Antwort

Gibt zurück: [`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## Beispiel

[inline-code-attrs-start title = 'updateUserBadge Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateUserBadgeParams(
  name = "Top Contributor",
  description = "Awarded for consistent high-quality comments",
  active = true,
  tags = @["community", "milestone"]
)

let (response, httpResponse) = client.updateUserBadge(tenantId = "my-tenant-123", id = "badge-456", updateUserBadgeParams = updateParams)

if response.isSome:
  let updated = response.get()
  echo "Badge updated successfully"
else:
  echo "Failed to update badge, HTTP status: ", $httpResponse.status
[inline-code-end]

---