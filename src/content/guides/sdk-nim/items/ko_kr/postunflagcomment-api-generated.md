## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'postUnFlagComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postUnFlagComment(commentId = "comment-8f3a2b4e", sso = "")
if response.isSome:
  let apiEmpty = response.get()
  echo "Comment unflagged successfully, response: ", apiEmpty
else:
  echo "Failed to unflag comment. HTTP response: ", httpResponse
[inline-code-end]

---