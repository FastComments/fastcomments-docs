## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostRemoveCommentOptions | No |  |

## 응답

Returns: [`Option[PostRemoveCommentApiResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_api_response.nim)

## 예시

[inline-code-attrs-start title = 'postRemoveComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponseOpt, httpResp) = client.postRemoveComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = PostRemoveCommentOptions()
)

if apiResponseOpt.isSome:
  let apiResponse = apiResponseOpt.get()
[inline-code-end]