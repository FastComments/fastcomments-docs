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
const tenantId: string = 'tenant_7f4b2a1c';
const pageSize: number = 25;
const afterId: string | undefined = undefined;
const includeContext: boolean = true;
const afterCreatedAt: number | undefined = Date.now() - 7 * 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean | undefined = undefined;
const includeTranslations: boolean = true;
const sso: string | undefined = 'sso_jwt_eyJhbGci';
const response: GetUserNotifications200Response = await getUserNotifications(
  tenantId,
  pageSize,
  afterId,
  includeContext,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  includeTranslations,
  sso
);
[inline-code-end]
