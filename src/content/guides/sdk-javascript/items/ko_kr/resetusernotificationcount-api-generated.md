## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
  const tenantId: string = 'tenant_84f3b2';
  const resetResultNoSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);

  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const resetResultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);

  console.log(resetResultNoSso, resetResultWithSso);
}

example();
[inline-code-end]

---