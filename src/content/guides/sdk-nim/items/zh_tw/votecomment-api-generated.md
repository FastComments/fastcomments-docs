## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 否 |  |
| voteBodyParams | VoteBodyParams | 否 |  |
| options | VoteCommentOptions | 否 |  |

## 回應

返回: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## 範例

[inline-code-attrs-start title = 'voteComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteRespOpt, httpResp) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  urlId = "blog/how-to-code",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  options = VoteCommentOptions()
)

if voteRespOpt.isSome:
  let voteResp = voteRespOpt.get()
[inline-code-end]

---