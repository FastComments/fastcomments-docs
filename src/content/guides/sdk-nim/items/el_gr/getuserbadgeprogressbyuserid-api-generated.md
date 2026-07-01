## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[APIGetUserBadgeProgressResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadgeProgressByUserId'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (badgeProgressOpt, httpResp) = client.getUserBadgeProgressByUserId(tenantId = "my-tenant-123", userId = "user-456")
if badgeProgressOpt.isSome:
  let progress = badgeProgressOpt.get()
  echo progress
[inline-code-end]