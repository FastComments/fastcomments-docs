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
const tenantId: string = 'tenant_7b9c2a';
const id: string = 'comment_4f8e1d';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'User submitted appeal and provided additional context',
  effectiveAt: new Date().toISOString()
};
const userId: string = 'user_92a3f6';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---