---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateNotificationBody | UpdateNotificationBody | はい |  |
| userId | string | いいえ |  |

## レスポンス

返り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'updateNotification の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2c';
const id: string = 'notification_4a1d2e';
const updateNotificationBody: UpdateNotificationBody = {
  enabled: true,
  channels: ['email', 'push'],
  frequency: 'immediate',
  templateId: 'tmpl_77aa'
} as UpdateNotificationBody;
const userId: string = 'user_2468';
const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
[inline-code-end]

---