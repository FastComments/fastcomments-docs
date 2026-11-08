## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| urlId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| voteBodyParams | VoteBodyParams | हाँ |  |
| sessionId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'voteComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVote() {
  const tenantId: string = 'tenant_001';
  const commentId: string = 'cmt_987654';
  const urlId: string = 'url_12345';
  const broadcastId: string = 'brd_56789';
  const voteBody: VoteBodyParams = { direction: 'up' };
  const sessionId: string = 'sess_abc123';
  const sso: string = 'sso_token_xyz';
  const response: VoteResponse = await voteComment(
    tenantId,
    commentId,
    urlId,
    broadcastId,
    voteBody,
    sessionId,
    sso
  );
  console.log(response);
}
runVote();
[inline-code-end]