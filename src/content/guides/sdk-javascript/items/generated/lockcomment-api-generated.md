## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## Example

[inline-code-attrs-start title = 'lockComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_987';
const commentId: string = 'comment_10293';
const broadcastId: string = 'broadcast_live_20251122';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyMyIsImlhdCI6MTY5OTk5OTk5OX0.signature';

const lockResultNoSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
const lockResultWithSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]
