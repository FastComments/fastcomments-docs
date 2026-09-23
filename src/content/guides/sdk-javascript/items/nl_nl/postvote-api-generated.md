## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| direction | string | Nee |  |
| broadcastId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postVote Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVoteExamples(): Promise<void> {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Aanroep met alleen verplichte parameters
  const simpleVote: VoteResponse = await postVote(tenantId, commentId);

  // Aanroep met optionele parameters
  const direction: string = "down";
  const broadcastId: string = "brd_987654321";
  const sso: string = "user-42";
  const detailedVote: VoteResponse = await postVote(tenantId, commentId, direction, broadcastId, sso);

  console.log(simpleVote, detailedVote);
}
[inline-code-end]