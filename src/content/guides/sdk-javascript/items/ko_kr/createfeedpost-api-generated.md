## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createFeedPostParams | CreateFeedPostParams | 예 |  |
| broadcastId | string | 아니오 |  |
| isLive | boolean | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| skipDupCheck | boolean | 아니오 |  |

## 응답

반환: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPost200Response.ts)

## 예제

[inline-code-attrs-start title = 'createFeedPost 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f3b2';
const mediaAsset: FeedPostMediaItemAsset = { url: 'https://cdn.example.com/images/post-123.jpg', mimeType: 'image/jpeg', width: 1200, height: 800, size: 245000 };
const mediaItem: FeedPostMediaItem = { id: 'media_1', type: 'image', assets: [mediaAsset], altText: 'Conference keynote stage' };
const link: FeedPostLink = { url: 'https://news.example.com/keynote-recap', title: 'Keynote recap' };
const createFeedPostParams: CreateFeedPostParams = {
  title: 'Product Launch Highlights',
  content: 'Highlights from today’s product launch and roadmap updates.',
  authorId: 'user_42',
  mediaItems: [mediaItem],
  links: [link],
  tags: ['product', 'launch', 'announcement']
};
const broadcastId: string = 'broadcast_20260424';
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const result: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]

---