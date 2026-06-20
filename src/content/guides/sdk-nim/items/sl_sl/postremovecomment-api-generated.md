## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`Option[PostRemoveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer postRemoveComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postRemoveComment(commentId = "cmt-987654321", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.abc123.signature")
if response.isSome:
  let removed = response.get()
  echo "Comment removed:", removed
else:
  echo "Failed to remove comment, HTTP response:", httpResponse
[inline-code-end]

---