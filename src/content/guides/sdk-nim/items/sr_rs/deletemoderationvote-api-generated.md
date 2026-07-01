## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | No |  |
| options | DeleteModerationVoteOptions | No |  |

## Odgovor

Vraća: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Primer

[inline-code-attrs-start title = 'deleteModerationVote Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteModerationVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  voteId = "vote-abc123",
  options = DeleteModerationVoteOptions()
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]