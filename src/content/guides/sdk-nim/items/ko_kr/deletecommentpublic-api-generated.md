## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| options | DeleteCommentPublicOptions | No |  |

## 응답

반환: [`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteCommentPublic 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResp) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  options = DeleteCommentPublicOptions())
if responseOpt.isSome:
  let resp = responseOpt.get()
  echo resp
[inline-code-end]