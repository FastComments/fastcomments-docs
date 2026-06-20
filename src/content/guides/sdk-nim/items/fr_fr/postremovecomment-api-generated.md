## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[PostRemoveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de postRemoveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postRemoveComment(commentId = "cmt-987654321", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.abc123.signature")
if response.isSome:
  let removed = response.get()
  echo "Comment removed:", removed
else:
  echo "Failed to remove comment, HTTP response:", httpResponse
[inline-code-end]

---