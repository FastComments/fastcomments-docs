## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | No |  |
| options | SaveCommentOptions | No |  |

## 응답

반환: [`Option[APISaveCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_save_comment_response.nim)

## 예시

[inline-code-attrs-start title = 'saveComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentParams = CreateCommentParams(
  body: "Great read, thanks!",
  name: "Alice Smith",
  email: "alice@example.com",
  parentId: ""
)

let saveOpts = SaveCommentOptions(
  isPreview: false,
  isApproved: true,
  skipSpamCheck: false
)

let (apiResponse, httpResponse) = client.saveComment(
  tenantId = "my-tenant-123",
  createCommentParams = commentParams,
  options = saveOpts
)

if apiResponse.isSome:
  let saved = apiResponse.get()
  echo saved
[inline-code-end]