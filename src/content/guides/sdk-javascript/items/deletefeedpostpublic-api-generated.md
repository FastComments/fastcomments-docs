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
const tenantId: string = "tenant_8f3a2b4c";
const postId: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const broadcastId: string | undefined = "broadcast_2026-01-09_19";
const sso: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature";
const result: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]
