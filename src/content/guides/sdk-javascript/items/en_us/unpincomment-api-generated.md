## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Example

[inline-code-attrs-start title = 'unPinComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2a3b';
const commentId: string = 'comment_842b9c1f';
const broadcastId: string = 'bcast_frontpage_202603';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const result: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]
