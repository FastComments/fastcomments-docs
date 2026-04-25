## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| blockFromCommentParams | BlockFromCommentParams | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳：[`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'blockUserFromComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---