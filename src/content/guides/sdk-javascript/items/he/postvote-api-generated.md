## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| direction | string | לא |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // קריאה רק עם פרמטרים נדרשים
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // קריאה עם פרמטרים אופציונליים
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]

---