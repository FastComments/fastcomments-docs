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

[inline-code-attrs-start title = 'unLockComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9d4f2b';
const commentId: string = 'cmt_8a3e1f';
const broadcastId: string = 'broadcast_2026_03_25';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';

const result: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]
