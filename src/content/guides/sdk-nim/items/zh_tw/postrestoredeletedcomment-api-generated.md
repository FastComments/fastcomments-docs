## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| options | PostRestoreDeletedCommentOptions | 否 |  |

## 回應

返回: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'postRestoreDeletedComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.postRestoreDeletedComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = default(PostRestoreDeletedCommentOptions)
)

if respOpt.isSome:
  let empty = respOpt.get()
  echo "Comment restored"
[inline-code-end]