## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| options | GetCommentTextOptions | 아니오 |  |

## 응답

반환: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## 예시

[inline-code-attrs-start title = 'getCommentText 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = GetCommentTextOptions()
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
[inline-code-end]