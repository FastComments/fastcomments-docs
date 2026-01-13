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
const tenantId: string = 'acme-news';
const urlIdWS: string = 'wss://ws.fastcomments.com/tenant/acme-news';
const userIds: string = 'user-7a9b,user-11f2';
type PresenceOptions = { includeInactive?: boolean };
const options: PresenceOptions = { includeInactive: true };
const presence: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]
