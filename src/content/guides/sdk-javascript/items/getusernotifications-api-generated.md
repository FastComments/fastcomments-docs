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
const tenantId: string = 'tenant_42_prod';
const pageSize: number = 50;
const includeContext: boolean = true;
const afterCreatedAt: number = Date.now() - 1000 * 60 * 60 * 24; // 24 hours ago
const unreadOnly: boolean = true;
const includeTranslations: boolean = false;
const sso: string = 'user-12345@company.com';
const notifications: GetUserNotifications200Response = await getUserNotifications(
  tenantId,
  pageSize,
  undefined,
  includeContext,
  afterCreatedAt,
  unreadOnly,
  undefined,
  undefined,
  includeTranslations,
  sso
);
[inline-code-end]
