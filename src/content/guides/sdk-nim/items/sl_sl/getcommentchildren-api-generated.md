## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Primer

[inline-code-attrs-start title = 'getCommentChildren Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentChildren(commentId = "comment-98765", sso = "")
if response.isSome:
  let childResp = response.get()
  discard childResp
[inline-code-end]

---