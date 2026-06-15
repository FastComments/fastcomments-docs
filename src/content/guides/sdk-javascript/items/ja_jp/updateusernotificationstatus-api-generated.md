## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| notificationId | string | はい |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2a8b9c';
const notificationId: string = 'notif_987654321';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---