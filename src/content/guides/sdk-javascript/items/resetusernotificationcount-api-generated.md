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
const tenantId: string = "a1b2c3d4-5678-90ab-cdef-1234567890ab";
const resultWithoutSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);

const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjM0IiwiaWF0IjoxNjM0MjM0NTYwfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const resultWithSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
[inline-code-end]
