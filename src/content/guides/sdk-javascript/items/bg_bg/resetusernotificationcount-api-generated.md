## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Отговор

Връща: [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## Пример

[inline-code-attrs-start title = 'resetUserNotificationCount пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // Извикване с изборен sso параметър
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // Извикване без изборен sso параметър
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]