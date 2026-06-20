## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| commentId | string | 예 |  |
| setCommentTextParams | SetCommentTextParams | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## 예제

[inline-code-attrs-start title = 'postSetCommentText 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentText(commentId = "comment-4821",
  setCommentTextParams = SetCommentTextParams(text = "Updated comment to clarify the main point and fix a typo."),
  sso = "sso-user-8f3b9c")

if response.isSome:
  let setCommentResp = response.get()
  echo "Received SetCommentTextResponse"
[inline-code-end]

---