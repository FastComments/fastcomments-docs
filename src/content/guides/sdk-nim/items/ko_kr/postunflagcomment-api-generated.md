## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostUnFlagCommentOptions | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'postUnFlagComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.postUnFlagComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  options = default(PostUnFlagCommentOptions)
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]