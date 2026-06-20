## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[PostRemoveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_response.nim)

## 예제

[inline-code-attrs-start title = 'postRemoveComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postRemoveComment(commentId = "cmt-987654321", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.abc123.signature")
if response.isSome:
  let removed = response.get()
  echo "Comment removed:", removed
else:
  echo "Failed to remove comment, HTTP response:", httpResponse
[inline-code-end]

---