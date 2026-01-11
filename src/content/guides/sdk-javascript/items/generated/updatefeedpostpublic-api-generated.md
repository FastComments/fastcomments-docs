## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const postId: string = "post_2025-11-22-001";
const updateFeedPostParams: UpdateFeedPostParams = {
  title: "Service Window: Brief Maintenance",
  content: "We will perform a brief maintenance window on 2025-11-23 at 02:00 UTC. Expected downtime ~5 minutes.",
  media: [{ id: "m_1001", type: "image", assets: [{ url: "https://cdn.acme.com/images/maintenance-banner.jpg", width: 1200, height: 300 }] }],
  links: [{ url: "https://status.acme.com/2025-11-23", title: "Detailed status" }],
  isPublished: true
};
const broadcastId: string = "broadcast_9f8b7c6d";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature";
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]
