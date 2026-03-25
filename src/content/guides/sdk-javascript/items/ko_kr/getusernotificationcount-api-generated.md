## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = '9f1e2d3c-4b5a-6d7e-8f90-123456789abc';
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI0MjMifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const resultWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
  console.log(resultWithSSO, resultWithoutSSO);
})();
[inline-code-end]