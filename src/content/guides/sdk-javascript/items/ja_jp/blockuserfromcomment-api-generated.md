## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| blockFromCommentParams | BlockFromCommentParams | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'blockUserFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'comment_7f3b2a9c';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'Repeated abusive language and targeted harassment',
  durationDays: 90,
  preventReposting: true
};
const userId: string = 'user_12345';
const anonUserId: string = 'anon_98765';

const result: BlockFromCommentPublic200Response = await blockUserFromComment(
  tenantId,
  id,
  blockFromCommentParams,
  userId,
  anonUserId
);
[inline-code-end]