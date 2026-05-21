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
  const tenantId: string = 'acme-tenant-78f3';
  const noSsoResult: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const withSsoResult: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
  console.log(noSsoResult, withSsoResult);
})();
[inline-code-end]
