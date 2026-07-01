## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentsByIdsParams | CommentsByIdsParams | No |  |
| sso | string = "" | No |  |

## 回應

返回：[`Option[ModerationAPIChildCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_child_comments_response.nim)

## 範例

[inline-code-attrs-start title = 'postCommentsByIds 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let params = CommentsByIdsParams(commentIds = @["cmt-001", "cmt-002"])
let (maybeResp, httpResp) = client.postCommentsByIds(tenantId = tenantId, commentsByIdsParams = params, sso = "")
if maybeResp.isSome:
  let resp = maybeResp.get()
  # use resp as needed
[inline-code-end]