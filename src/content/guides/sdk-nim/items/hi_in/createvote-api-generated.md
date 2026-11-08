## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| direction | string | नहीं |  |
| options | CreateVoteOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'createVote उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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