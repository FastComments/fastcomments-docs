## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| editKey | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteCommentVote Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // optioneel
const ssoToken: string = "sso_token_xyz"; // optioneel

// Aanroep met alleen de vereiste parameters en één optioneel
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Aanroep met beide optionele parameters
const deleteResultWithSSO: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  ssoToken
);
[inline-code-end]