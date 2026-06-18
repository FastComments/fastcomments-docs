## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 是 |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2a8b9c';
const notificationId: string = 'notif_987654321';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---