## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | string | No |  |
| options | CreateVoteOptions | No |  |

## レスポンス

返り値: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## 例

[inline-code-attrs-start title = 'createVote 例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  direction = "up",
  options = CreateVoteOptions()
)

if voteOpt.isSome:
  let vote = voteOpt.get()
  echo vote
[inline-code-end]