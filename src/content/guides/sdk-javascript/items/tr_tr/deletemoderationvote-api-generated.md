## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| voteId | string | Evet |  |
| broadcastId | string | Hayır |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`DeleteModerationVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModerationVoteResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteModerationVote Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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