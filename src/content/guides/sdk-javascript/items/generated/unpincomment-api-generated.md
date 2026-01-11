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
const tenantId: string = 'tenant_84a2c1';
const commentId: string = 'cmt-9f2b6d7a';
const broadcastId: string = 'live-20251122-nyc';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlcjEyMyIsImlhdCI6MTY5MDUwMDAwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const responseWithSso: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, ssoToken);
const responseWithoutSso: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId);
[inline-code-end]
