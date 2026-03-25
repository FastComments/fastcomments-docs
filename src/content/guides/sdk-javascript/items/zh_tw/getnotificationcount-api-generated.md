## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | boolean | 否 |  |
| type | string | 否 |  |

## 回應

回傳：[`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCount200Response.ts)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_abc123';
const userId: string = 'user_987654321';
const urlId: string = 'https://example.com/news/2026/new-features';
const viewed: boolean = false;
const type: string = 'reply';
const notificationCountResponse: GetNotificationCount200Response = await getNotificationCount(tenantId, userId, urlId, undefined, viewed, type);
[inline-code-end]

---