## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string = "" | No |  |

## Απόκριση

Επιστρέφει: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'blockFromCommentPublic Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (blockOpt, httpResp) = client.blockFromCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if blockOpt.isSome:
  let block = blockOpt.get()
[inline-code-end]