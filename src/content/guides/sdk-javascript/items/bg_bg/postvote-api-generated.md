## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | string | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Отговор

Връща: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'postVote Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Извикване само с задължителните параметри
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // Извикване с незадължителни параметри
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]

---