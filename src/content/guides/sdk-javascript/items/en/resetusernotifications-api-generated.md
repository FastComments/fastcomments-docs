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
const params: {
  tenantId: string;
  afterId?: string;
  afterCreatedAt?: number;
  unreadOnly?: boolean;
  dmOnly?: boolean;
  noDm?: boolean;
  sso?: string;
} = {
  tenantId: 'tenant_7f3b9c',
  afterId: 'notif_20260520_01',
  afterCreatedAt: Date.now() - 3_600_000,
  unreadOnly: true,
  sso: 'sso_session_xyz'
};
const result: ResetUserNotifications200Response = await resetUserNotifications(params);
[inline-code-end]
