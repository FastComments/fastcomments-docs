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

Returns: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'site_10294';
const afterId: string = 'notification_987654321';
const afterCreatedAt: number = 1672531200000;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const sso: string = 'sso_user_7f2b';

const result: ResetUserNotifications200Response = await resetUserNotifications(
  tenantId,
  afterId,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  sso
);
[inline-code-end]
