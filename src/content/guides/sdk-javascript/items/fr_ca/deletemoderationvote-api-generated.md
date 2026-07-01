## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Réponse

Renvoie : [`DeleteModerationVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModerationVoteResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteModerationVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const voteId: string = "vote_9876";
const broadcastId: string = "brd_001";
const tenantId: string = "tenant_42";
const sso: string = "sso_token_abc";

const result: DeleteModerationVoteResponse = await deleteModerationVote(
  commentId,
  voteId,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]