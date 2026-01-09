## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| isFlagged | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primer

[inline-code-attrs-start title = 'flagCommentPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  isFlagged = false,
  sso = ""
)
if response.isSome:
  let flagResult = response.get()
  discard flagResult
[inline-code-end]

---