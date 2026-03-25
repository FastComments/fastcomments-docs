---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 使用例

[inline-code-attrs-start title = 'unBlockCommentPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42e8a1';
const commentId: string = 'cmt_9b3f2d';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'abusive_language',
  blockedByModeratorId: 'mod_17',
  note: 'Targeted harassment; review complete',
  unblockRequestedAt: new Date().toISOString()
};
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---