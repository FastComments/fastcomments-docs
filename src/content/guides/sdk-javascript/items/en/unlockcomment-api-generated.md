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
const tenantId: string = "tenant_7b3f2d";
const commentId: string = "cmt-92f4b1";
const broadcastId: string = "brd-20260520-01";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1NjciLCJpYXQiOjE2ODYyMzQ1MDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const resultWithoutSSO: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId);
const resultWithSSO: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]
