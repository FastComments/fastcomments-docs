## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| pageSize | number | No |  |
| afterId | string | No |  |
| includeContext | boolean | No |  |
| afterCreatedAt | number | No |  |
| unreadOnly | boolean | No |  |
| dmOnly | boolean | No |  |
| noDm | boolean | No |  |
| includeTranslations | boolean | No |  |
| sso | string | No |  |

## Response

Returns: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserNotifications Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const notifications: GetUserNotifications200Response = await getUserNotifications({
  tenantId: "tenant_01a9f3",
  pageSize: 25,
  afterId: "notif_987654321",
  includeContext: true,
  afterCreatedAt: Date.now() - 3600_000,
  unreadOnly: true,
  dmOnly: false,
  includeTranslations: true,
  sso: "sso-token-abcdef123456"
});
[inline-code-end]
