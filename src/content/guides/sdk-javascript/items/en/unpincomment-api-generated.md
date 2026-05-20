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
const tenantId: string = "a3f9c6e2-1d5b-4f2a-9c8e-2b7ef0c3a1d4";
const commentId: string = "cmt-987654321";
const broadcastId: string = "bcast-2026-05-20-01";
const sso: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSJ9.signature";
const response: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]
