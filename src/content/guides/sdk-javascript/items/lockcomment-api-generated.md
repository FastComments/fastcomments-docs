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
const tenantId: string = 'tenant_4127';
const commentId: string = 'cmt_00123456789';
const broadcastId: string = 'bcast_nfl_20260109';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTY5MDAwMDB9.signature';

const lockResultWithSSO: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
const lockResultWithoutSSO: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]
