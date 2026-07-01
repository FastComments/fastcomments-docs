## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentIds | string | Όχι |  |
| sso | string = "" | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'checkedCommentsForBlocked Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "cmt-1,cmt-2",
  sso = ""
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
  discard response
[inline-code-end]

---