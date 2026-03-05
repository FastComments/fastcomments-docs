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
const tenantId: string = 'acme-tenant-7a1f2b';
const commentId: string = 'cmt-9d4e2f7a-12';
const broadcastId: string = 'bcast-2026-01-12-live';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4IiwiaWF0IjoxNjQwMDAwMDB9.signature';

const resultWithoutSso: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId);
const resultWithSso: LockComment200Response = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]
