## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 否 |  |
| voteBodyParams | VoteBodyParams | 否 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## 示例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  urlId = "news/article-2026-inflation",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  sessionId = "",
  sso = ""
)

if response.isSome:
  let voteResp = response.get()
  discard voteResp
else:
  discard httpResponse
[inline-code-end]

---