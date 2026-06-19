## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| notificationId | string | 예 |  |
| newStatus | UpdateUserNotificationStatusNewStatusEnum | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-8d3f2b7c';
const notificationId: string = 'notification-587a2b9f';
const newStatus: UpdateUserNotificationStatusNewStatusEnum = UpdateUserNotificationStatusNewStatusEnum.Read;
const sso: string = 'sso-token-1a2b3c4d5e6f';
const result: UpdateUserNotificationStatusResponse = await updateUserNotificationStatus(tenantId, notificationId, newStatus, sso);
[inline-code-end]

---