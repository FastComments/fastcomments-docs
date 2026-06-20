## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerationCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationCommentText(commentId = "comment-9f8b7a6c", sso = "")
if response.isSome:
  let commentData = response.get()
  echo "Moderation comment text retrieved"
else:
  echo "No moderation comment text available"
[inline-code-end]

---