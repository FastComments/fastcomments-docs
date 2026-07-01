## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 否 |  |
| options | SetCommentTextOptions | 否 |  |

## 回應

Returns: [`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## 範例

[inline-code-attrs-start title = 'setCommentText 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentUpdate = CommentTextUpdateRequest(text: "Updated comment text")
let opts = SetCommentTextOptions()
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  broadcastId = "broadcast-789",
  commentTextUpdateRequest = commentUpdate,
  options = opts)
if response.isSome:
  let result = response.get()
[inline-code-end]