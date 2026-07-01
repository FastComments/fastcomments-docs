## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-------------|
| tenantId | string | Ναι |  |
| createModeratorBody | CreateModeratorBody | Όχι |  |

## Ανταπόκριση

Επιστρέφει: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]