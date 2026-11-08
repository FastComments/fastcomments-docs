## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| voteBodyParams | VoteBodyParams | नहीं |  |
| options | VoteCommentOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'voteComment उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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