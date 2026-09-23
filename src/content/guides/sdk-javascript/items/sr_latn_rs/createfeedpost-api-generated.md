## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| createFeedPostParams | CreateFeedPostParams | Da |  |
| broadcastId | string | Ne |  |
| isLive | boolean | Ne |  |
| doSpamCheck | boolean | Ne |  |
| skipDupCheck | boolean | Ne |  |

## Odgovor

Vraća: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostsResponse.ts)

## Primer

[inline-code-attrs-start title = 'createFeedPost Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
  const broadcastId: string = "b1c2d3e4-5678-90ab-cdef-1234567890cd";
  const isLive: boolean = true;
  const doSpamCheck: boolean = false;
  const skipDupCheck: boolean = true;

  const mediaAsset: FeedPostMediaItemAsset = {
    url: "https://cdn.example.com/photos/sunrise.jpg",
    type: "image/jpeg"
  };
  const mediaItem: FeedPostMediaItem = {
    asset: mediaAsset,
    caption: "Sunrise over the mountains"
  };
  const linkItem: FeedPostLink = {
    url: "https://blog.example.com/tech-trends-2024",
    title: "Top Tech Trends for 2024"
  };
  const postParams: CreateFeedPostParams = {
    text: "Excited to share the latest tech trends!",
    media: [mediaItem],
    link: linkItem
  };

  const response: CreateFeedPostsResponse = await createFeedPost(
    tenantId,
    postParams,
    broadcastId,
    isLive,
    doSpamCheck,
    skipDupCheck
  );
  console.log(response);
})();
[inline-code-end]