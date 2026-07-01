## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| commentId | string | Ja |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Ja |  |
| broadcastId | string | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`PostAdjustCommentVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostAdjustCommentVotesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postAdjustCommentVotes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8b7a6d";

const adjustParams: AdjustCommentVotesParams = {
  voteDelta: 1,
  // extra velden zoals vereist door AdjustCommentVotesParams
};

const broadcastId: string = "brd_20230915";
const tenantId: string = "tenant_42";
const sso: string = "sso-token-abc123";

const result: PostAdjustCommentVotesResponse = await postAdjustCommentVotes(
  commentId,
  adjustParams,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]