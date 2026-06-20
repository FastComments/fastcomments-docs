---
## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne: [`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentChildren'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentChildren(commentId = "comment-98765", sso = "")
if response.isSome:
  let childResp = response.get()
  discard childResp
[inline-code-end]

---