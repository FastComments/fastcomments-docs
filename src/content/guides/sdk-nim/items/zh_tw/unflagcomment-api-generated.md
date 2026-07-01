## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | UnFlagCommentOptions | No |  |

## 回應

返回：[`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 範例

[inline-code-attrs-start title = 'unFlagComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (flagRespOpt, httpResp) = client.unFlagComment(tenantId = "my-tenant-123", id = "comment-456", options = UnFlagCommentOptions())
if flagRespOpt.isSome:
  let flagResp = flagRespOpt.get()
[inline-code-end]