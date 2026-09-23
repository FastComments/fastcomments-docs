## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| voteId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| editKey | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteCommentVote Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // optional
const ssoToken: string = "sso_token_xyz"; // optional

// Aufruf nur mit den erforderlichen Parametern und einem optionalen Parameter
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Aufruf mit beiden optionalen Parametern
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

---