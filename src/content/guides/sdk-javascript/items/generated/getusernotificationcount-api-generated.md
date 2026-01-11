## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserNotificationCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-42';
const ssoToken: string = 'sso:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example';
const notificationsWithoutSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
const notificationsWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
[inline-code-end]
