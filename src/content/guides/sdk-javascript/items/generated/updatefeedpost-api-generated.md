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
const tenantId: string = 'acme-enterprises-42';
const id: string = 'feedpost_20251122_001';
const feedPost: FeedPost = {
  title: 'Product roadmap update',
  content: 'Updated roadmap including new moderation and analytics features.',
  authorName: 'Acme Product Team',
  isPublished: false,
  scheduledAt: '2025-12-01T09:00:00Z', // optional scheduling parameter
  media: [
    {
      type: 'image',
      url: 'https://assets.acme.com/images/roadmap.png',
      assets: [{ resolution: '1024x768', url: 'https://assets.acme.com/images/roadmap-1024.png' }]
    }
  ],
  links: [{ title: 'Full Roadmap', url: 'https://acme.com/roadmap' }],
  tags: ['roadmap', 'release', 'moderation']
};
const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]
