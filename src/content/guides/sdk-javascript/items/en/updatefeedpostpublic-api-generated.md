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
const tenantId: string = 'tenant_acme_prod';
const postId: string = 'post_20260520_01';
const mediaAsset: FeedPostMediaItemAsset = { url: 'https://cdn.acme.com/images/maintenance.png', mimeType: 'image/png' };
const mediaItem: FeedPostMediaItem = { asset: mediaAsset };
const link: FeedPostLink = { url: 'https://status.acme.com/incidents/54321', title: 'Incident details' };
const updateFeedPostParams: UpdateFeedPostParams = {
  title: 'Scheduled maintenance',
  content: 'Maintenance on June 30, 02:00–04:00 UTC. Brief service interruptions possible.',
  mediaItems: [mediaItem],
  links: [link]
};
const broadcastId: string | undefined = 'broadcast_9988';
const sso: string | undefined = 'sso_jwt_token_eyJhbGci';
const result: CreateFeedPostPublic200Response = await updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso);
[inline-code-end]
