## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| direction | string | いいえ |  |
| broadcastId | string | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`PostVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostVoteResponse.ts)

## 例

[inline-code-attrs-start title = 'postVote の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentId: string = 'cmt_12345';
let direction: string = 'up';
let broadcastId: string = 'brd_67890';
let tenantId: string = 'tenant_abc';
let sso: string = 'sso_token_xyz';

const fullVote: PostVoteResponse = await postVote(
  commentId,
  direction,
  broadcastId,
  tenantId,
  sso
);

const simpleVote: PostVoteResponse = await postVote('cmt_67890');
[inline-code-end]