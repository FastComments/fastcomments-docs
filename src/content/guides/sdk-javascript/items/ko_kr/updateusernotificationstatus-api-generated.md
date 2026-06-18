## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| notificationId | string | 예 |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2a8b9c';
const notificationId: string = 'notif_987654321';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---