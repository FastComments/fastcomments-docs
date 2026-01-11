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
const tenantId: string = 'tenant_acme_2025';
const postId: string = 'post_9f8a7b6c';
const broadcastId: string = 'broadcast_us_east_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakeSignature';
const result: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]
