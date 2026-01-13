## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createUserBadgeParams | CreateUserBadgeParams | Non |  |

## Réponse

Renvoie : [`Option[CreateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_user_badge200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createUserBadge(tenantId = "my-tenant-123",
  createUserBadgeParams = CreateUserBadgeParams(userId = "user-987", badgeId = "top-commenter"))
if response.isSome:
  let createdBadge = response.get()
  echo "Created badge for user: ", createdBadge.userId
else:
  echo "Failed to create badge, status: ", $httpResponse.status
[inline-code-end]

---