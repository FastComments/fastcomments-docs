## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`DeleteModerationVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteModerationVoteResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteModerationVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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