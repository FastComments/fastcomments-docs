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
const tenantId: string = 'acme-corp-tenant-1';
const urlIdWS: string = 'wss://realtime.fastcomments.com/ws/site-acme';
const optionalExtraUserId: string | undefined = 'user_789'; // optional
const userIds: string = ['user_123', 'user_456', ...(optionalExtraUserId ? [optionalExtraUserId] : [])].join(',');
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
presenceStatuses;
[inline-code-end]
