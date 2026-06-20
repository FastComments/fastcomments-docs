## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Response

Renvoie: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let badgeProgress = response.get()
  echo badgeProgress
else:
  echo "No badge progress found"
[inline-code-end]

---