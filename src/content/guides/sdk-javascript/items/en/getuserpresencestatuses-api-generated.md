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
async function fetchPresence(tenantId: string, userIds: string, urlIdWS?: string): Promise<GetUserPresenceStatuses200Response> {
  const effectiveUrl: string = urlIdWS ?? 'wss://presence.prod.company.com/tenant_98765';
  return await getUserPresenceStatuses(tenantId, effectiveUrl, userIds);
}
const tenantId: string = 'tenant_98765';
const userIds: string = 'user-112233,user-445566';
const presence: GetUserPresenceStatuses200Response = await fetchPresence(tenantId, userIds);
[inline-code-end]
