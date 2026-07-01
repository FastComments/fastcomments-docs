## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απάντηση

Επιστρέφει: [`PostAdjustCommentVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostAdjustCommentVotesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postAdjustCommentVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8b7a6d";

const adjustParams: AdjustCommentVotesParams = {
  voteDelta: 1,
  // πρόσθετα πεδία όπως απαιτούνται από AdjustCommentVotesParams
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