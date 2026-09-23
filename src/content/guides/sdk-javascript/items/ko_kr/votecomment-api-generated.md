## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 예 |  |
| voteBodyParams | VoteBodyParams | 예 |  |
| sessionId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 예시

[inline-code-attrs-start title = 'voteComment 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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