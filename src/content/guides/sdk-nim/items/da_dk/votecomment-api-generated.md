## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Nej |  |
| voteBodyParams | VoteBodyParams | Nej |  |
| options | VoteCommentOptions | Nej |  |

## Svar

Returnerer: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Eksempel

[inline-code-attrs-start title = 'voteComment Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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