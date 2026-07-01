## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| options | GetModerationCommentOptions | Ne |  |

## Odgovor

Vraća: [`Option[ModerationAPICommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_comment_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getModerationComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeComment, httpResponse) = client.getModerationComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(GetModerationCommentOptions)
)

if maybeComment.isSome:
  let comment = maybeComment.get()
  echo comment
[inline-code-end]