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
const tenantId: string = "tenant_78f4b2";
const commentId: string = "comment_abc123xyz";
const broadcastId: string = "live_20260109_01";

// call without optional SSO
const unlockedWithoutSso: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId);

// call with optional SSO token
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyX2lkMTIzIiwiaWF0IjoxNjkyMDAwMDB9.dummySignature";
const unlockedWithSso: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]
