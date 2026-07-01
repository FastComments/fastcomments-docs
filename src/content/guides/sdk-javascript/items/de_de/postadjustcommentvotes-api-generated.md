## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| commentId | string | Ja |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Ja |  |
| broadcastId | string | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PostAdjustCommentVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostAdjustCommentVotesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postAdjustCommentVotes Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8b7a6d";

const adjustParams: AdjustCommentVotesParams = {
  voteDelta: 1,
  // zusätzliche Felder, wie von AdjustCommentVotesParams verlangt
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

---