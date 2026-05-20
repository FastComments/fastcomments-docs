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
const tenantId: string = "9f8b7c6d-1234-4b2a-9d8e-0a1b2c3d4e5f";
const urlIdWS: string = "wss://presencews.prod.company.example.com/stream";
const userIds: string = "a3e9b1f0-1111-4444-8888-1234567890ab,c2d3e4f5-2222-5555-9999-abcdefabcdef";
const includeOffline: boolean | undefined = true;

const result: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]
