## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Sì |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di postAdjustCommentVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_8f3a2b7d4e";
const adjustCommentVotesParams: AdjustCommentVotesParams = { delta: 1, reason: "useful", source: "web" } as AdjustCommentVotesParams;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature";
const result: AdjustVotesResponse = await postAdjustCommentVotes(commentId, adjustCommentVotesParams, sso);
[inline-code-end]

---