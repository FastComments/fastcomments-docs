## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| urlId | string | はい |  |
| broadcastId | string | はい |  |
| voteBodyParams | VoteBodyParams | はい |  |
| sessionId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## 例

[inline-code-attrs-start title = 'voteComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2e';
const commentId: string = '5a1d3f9b-2c4e-4a2b-bf7b-1234567890ab';
const urlId: string = 'articles/2026/06/15/typescript-api-patterns';
const broadcastId: string = 'broadcast-20260615-01';
const voteBodyParams: VoteBodyParams = { vote: 'up' };
const sessionId: string = 'sess_9d2f3b45';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyMyIsImlhdCI6MTY5NzE2MDAwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const response: VoteComment200Response = await voteComment(
  tenantId,
  commentId,
  urlId,
  broadcastId,
  voteBodyParams,
  sessionId,
  sso
);
[inline-code-end]