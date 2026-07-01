## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | No |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer unBlockCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (unblockResult, httpResp) = client.unBlockCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if unblockResult.isSome:
  let result = unblockResult.get()
[inline-code-end]

---