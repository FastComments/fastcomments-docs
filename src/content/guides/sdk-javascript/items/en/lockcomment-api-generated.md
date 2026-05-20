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
const tenantId: string = 'tenant-8423';
const commentId: string = '3fa85f64-5717-4562-b3fc-2c963f66afa6';
const broadcastId: string = 'broadcast-110';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const lockedWithSSO: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
const lockedWithoutSSO: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]
