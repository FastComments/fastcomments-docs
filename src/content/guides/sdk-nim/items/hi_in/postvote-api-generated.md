## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | PostVoteOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'postVote उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.postVote(
  tenantId = "my-tenant-123",
  commentId = "comment-789",
  options = default(PostVoteOptions)
)

if voteOpt.isSome:
  let vote = voteOpt.get()
[inline-code-end]