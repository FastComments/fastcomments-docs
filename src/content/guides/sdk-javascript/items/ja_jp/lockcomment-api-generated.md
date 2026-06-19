---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'lockComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-214';
const commentId: string = 'cmt_4f3b9a2d';
const broadcastId: string = 'live-987654321';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzgiLCJuYW1lIjoiSmFuZSBEb2UifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const lockedWithSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId, sso);
const lockedWithoutSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---