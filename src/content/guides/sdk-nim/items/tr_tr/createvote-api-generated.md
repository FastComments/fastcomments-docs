## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| direction | string | Hayır |  |
| options | CreateVoteOptions | Hayır |  |

## Yanıt

Döndürür: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Örnek

[inline-code-attrs-start title = 'createVote Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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