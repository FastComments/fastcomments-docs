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
const tenantId: string = 'fastcomments-tenant-84';
const createFeedPostParams: CreateFeedPostParams = {
  title: 'Product Update — Performance Improvements',
  html: '<p>We deployed several backend optimizations reducing page load by ~30%.</p>',
  authorName: 'Engineering Team',
  published: true,
  media: [
    {
      url: 'https://cdn.fastcomments.example/assets/release-2-4.png',
      type: 'image',
      assets: [{ url: 'https://cdn.fastcomments.example/assets/release-2-4-thumb.png', width: 320 }]
    } as FeedPostMediaItem
  ] as FeedPostMediaItem[],
  links: [{ url: 'https://fastcomments.example/changelog/2.4', title: 'Full changelog' } as FeedPostLink]
} as CreateFeedPostParams;
const broadcastId: string = 'broadcast_2026_01_12_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]
