## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlIdWS | string | Yes |  |
| userIds | string | Yes |  |

## Response

Returns: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserPresenceStatuses Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-842";
const urlIdWS: string = "article-9f3b2c";
const optionalUserIds: string | undefined = "u12345,u67890"; // optional: could be undefined to fallback
const userIds: string = optionalUserIds ?? "u12345";
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
presenceStatuses;
[inline-code-end]
