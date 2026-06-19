## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 是 |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-8d3f2b7c';
const notificationId: string = 'notification-587a2b9f';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'sso-token-1a2b3c4d5e6f';
const result: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---