## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateNotificationBody | UpdateNotificationBody | 是 |  |
| userId | string | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateNotification 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8f4b2c';
const id: string = 'notification_61a2e9';
const userId: string = 'moderator_107';
const updateNotificationBody: UpdateNotificationBody = {
  name: 'Flagged Comment Notification',
  enabled: true,
  channels: ['email', 'inbox'],
  templateId: 'tmpl_mod_alerts_01',
  severity: 'high'
};
const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
[inline-code-end]

---