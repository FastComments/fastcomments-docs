---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadgeProgressById'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressById(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let badgeProgress = response.get()
  echo badgeProgress
else:
  echo "No badge progress found"
[inline-code-end]

---