## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ne |  |
| sso | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Primjer

[inline-code-attrs-start title = 'unBlockCommentPublic Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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