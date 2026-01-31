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
const tenantId: string = 'tenant_84b3c2';
const commentId: string = 'cmt_20260109_001';
const broadcastId: string = 'brd_nyc_stream_01';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoSignature';

const pinResponseWithSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
const pinResponseWithoutSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
[inline-code-end]
