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
const tenantId: string = 'tenant_12345'
const commentId: string = 'comment-9b2d'
const broadcastId: string = 'broadcast-20260520-01'
const sso: string = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyX2lkMTIzIn0.D4kR7ZxVb3NqZ1xY'
const responseWithSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, sso)
const responseWithoutSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId)
[inline-code-end]
