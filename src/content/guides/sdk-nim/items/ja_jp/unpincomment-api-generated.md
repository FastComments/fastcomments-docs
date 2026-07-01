## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## レスポンス

返却: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## 例

[inline-code-attrs-start title = 'unPinComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.unPinComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "broadcast-001",
  sso = ""
)

if responseOpt.isSome:
  let resp = responseOpt.get()
  echo resp
[inline-code-end]