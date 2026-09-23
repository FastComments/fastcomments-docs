## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| direction | string | Nein |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Aufruf nur mit den erforderlichen Parametern
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // Aufruf mit optionalen Parametern
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]