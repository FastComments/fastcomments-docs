## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3a2b9c-4d5e-6f7a-8b9c-0d1e2f3a4b5c';
const postId: string = 'post_74b3c9d2-1a6f-4e5d-9c3b-2a1e0f6d7c8b';
const broadcastId: string = 'broadcast_2026-01-12_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InVzZXIxMjMiLCJleHAiOjE3MDAwMDAwMDB9.dummySignature12345';

const responseWithAll: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
const responseRequiredOnly: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId);
[inline-code-end]
