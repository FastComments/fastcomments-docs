## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Primjer

[inline-code-attrs-start title = 'unBlockCommentPublic Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(
  tenantId = "news-site-456",
  commentId = "comment-abc123",
  publicBlockFromCommentParams = PublicBlockFromCommentParams{},
  sso = ""
)
if response.isSome:
  let unblocked = response.get()
  discard unblocked
else:
  discard httpResponse
[inline-code-end]

---