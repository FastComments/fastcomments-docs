## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[PostRemoveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład postRemoveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postRemoveComment(commentId = "cmt-987654321", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.abc123.signature")
if response.isSome:
  let removed = response.get()
  echo "Comment removed:", removed
else:
  echo "Failed to remove comment, HTTP response:", httpResponse
[inline-code-end]