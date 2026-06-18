## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| urlId | string | いいえ |  |
| fromCommentId | string | いいえ |  |
| viewed | boolean | いいえ |  |
| type | string | いいえ |  |

## レスポンス

戻り値: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## 例

[inline-code-attrs-start title = 'getNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82a3b9f';
const userId: string = 'user_43721';
const urlId: string = 'https://news.example.com/articles/2026/06/15/coverage-123';
const fromCommentId: string = 'comment_98765';
const viewed: boolean = false;
const notificationType: string = 'mention';

const result: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, notificationType);
[inline-code-end]

---