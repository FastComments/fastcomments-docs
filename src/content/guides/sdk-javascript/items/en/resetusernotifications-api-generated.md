## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| afterCreatedAt | number | No |  |
| unreadOnly | boolean | No |  |
| dmOnly | boolean | No |  |
| noDm | boolean | No |  |
| sso | string | No |  |

## Response

Returns: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-9f2b';
const afterId: string = 'notif_7c1a2b3';
const afterCreatedAt: number = Date.now() - 3 * 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const sso: string = 'sso:microsoft:84012';
const response: ResetUserNotificationsResponse = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, undefined, undefined, sso);
[inline-code-end]
