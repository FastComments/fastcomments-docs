## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98765';
const createFeedPostParams: CreateFeedPostParams = {
  title: 'Weekly Product Update',
  body: 'Released performance improvements and bug fixes across the mobile app.',
  media: [
    {
      url: 'https://cdn.example.com/images/update-2026-05-20.jpg',
      type: 'image',
      assets: [{ url: 'https://cdn.example.com/images/update-2026-05-20.jpg', mimeType: 'image/jpeg' }]
    } as FeedPostMediaItem
  ],
  links: [{ url: 'https://example.com/release-notes/2026-05-20', title: 'Full release notes' } as FeedPostLink],
  tags: ['release', 'mobile', 'performance']
};
const broadcastId: string = 'broadcast_20260520';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]
