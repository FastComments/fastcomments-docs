## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'resetUserNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoReset() {
  const tenantId: string = "tenant_12345";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);
}

demoReset();
[inline-code-end]