## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| feedPost | FeedPost | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateFeedPost Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-01';
  const id: string = 'feedpost_20260109_001';
  const feedPost: FeedPost = {
    title: 'Scheduled maintenance: brief read-only period',
    body: 'Maintenance on 2026-01-12 02:00–04:00 UTC. Comment submission will be disabled during this window.',
    published: true,
    authorId: 'admin-42',
    media: [
      {
        type: 'image',
        caption: 'Maintenance banner',
        asset: {
          url: 'https://cdn.acme.com/images/maintenance-banner.png',
          mimeType: 'image/png',
          width: 1200,
          height: 400
        }
      }
    ],
    links: [{ title: 'Status page', url: 'https://status.acme.com' }]
  };
  const response: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
  console.log(response);
})();
[inline-code-end]
