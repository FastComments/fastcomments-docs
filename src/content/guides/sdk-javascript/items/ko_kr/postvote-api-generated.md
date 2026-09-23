## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | string | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## 예시

[inline-code-attrs-start title = 'postVote 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // 필수 매개변수만 사용하여 호출
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // 선택 매개변수와 함께 호출
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]