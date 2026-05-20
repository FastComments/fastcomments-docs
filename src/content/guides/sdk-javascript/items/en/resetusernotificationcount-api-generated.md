## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Example

[inline-code-attrs-start title = 'resetUserNotificationCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-enterprises-42";
  const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.e30.Kd3xYp1QeXl9Z8V7a2rNf4";
  const resultWithoutSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);
  const resultWithSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
  console.log(resultWithoutSSO, resultWithSSO);
})();
[inline-code-end]
