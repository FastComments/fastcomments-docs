## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'unBlockUserFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f3e1';
const id: string = 'comment_9b2a4f';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'Reviewed by moderation team — reinstated',
  moderatorId: 'mod_21',
  unblockedAt: new Date().toISOString()
};
const userId: string = 'user_42';
const anonUserId: string = 'anon_e7f9';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
[inline-code-end]