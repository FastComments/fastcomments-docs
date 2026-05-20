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

[inline-code-attrs-start title = 'pinComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_72f9';
const commentId: string = 'cmt_1a2b3c';
const broadcastId: string = 'live-2026-05-20-19';
const ssoToken: string = 'sso_tok_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken)
[inline-code-end]
