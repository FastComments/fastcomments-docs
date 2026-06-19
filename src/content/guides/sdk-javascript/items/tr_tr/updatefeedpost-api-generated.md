## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| feedPost | FeedPost | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateFeedPost Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-779';
const id: string = 'post_5f1b9c3a';
const asset: FeedPostMediaItemAsset = {
  url: 'https://cdn.acme.com/images/cover.jpg',
  type: 'image/jpeg',
  width: 1200,
  height: 800
};
const mediaItem: FeedPostMediaItem = {
  id: 'media_12f',
  caption: 'Event highlight',
  assets: [asset]
};
const link: FeedPostLink = {
  url: 'https://acme.com/events/2026',
  title: 'Event details'
};
const feedPost: FeedPost = {
  title: 'June Product Launch',
  content: '<p>Join us for the new product demo.</p>',
  published: true,
  media: [mediaItem],
  links: [link],
  authorId: 'user_93d'
};
const result: APIEmptyResponse = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]

---