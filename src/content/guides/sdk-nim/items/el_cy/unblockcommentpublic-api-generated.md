## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'unBlockCommentPublic Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", publicBlockFromCommentParams = PublicBlockFromCommentParams(), sso = "")
if response.isSome:
  let unblockResult = response.get()
  discard unblockResult
else:
  discard httpResponse
[inline-code-end]

---