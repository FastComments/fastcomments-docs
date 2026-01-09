## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[UpdateUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_badge200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if response.isSome:
  let updated = response.get()
  discard updated
[inline-code-end]

---