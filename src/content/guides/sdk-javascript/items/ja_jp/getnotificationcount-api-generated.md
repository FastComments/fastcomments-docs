## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| urlId | string | いいえ |  |
| fromCommentId | string | いいえ |  |
| viewed | boolean | いいえ |  |
| type | string | いいえ |  |

## レスポンス

返却値: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## 例

[inline-code-attrs-start title = 'getNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const userId: string = 'user_7421';
const urlId: string = 'https://news.example.com/articles/2026/06/19/ai-update';
const fromCommentId: string = 'cmt_5a1d2f';
const viewed: boolean = false;
const type: string = 'mention';

const notificationCount: GetNotificationCountResponse = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]

---