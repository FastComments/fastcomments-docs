## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| voteId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // προαιρετικό
const ssoToken: string = "sso_token_xyz"; // προαιρετικό

// Κλήση μόνο με τις απαιτούμενες παραμέτρους και μία προαιρετική
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Κλήση με και τις δύο προαιρετικές παραμέτρους
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