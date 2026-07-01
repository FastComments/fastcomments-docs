## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createFeedPostParams | CreateFeedPostParams | כן |  |
| broadcastId | string | לא |  |
| isLive | boolean | לא |  |
| doSpamCheck | boolean | לא |  |
| skipDupCheck | boolean | לא |  |

## תגובה

מחזיר: [`CreateFeedPostResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createFeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId = "tenant-12345";
  const feedParams: CreateFeedPostParams = {
    text: "New product launch!",
    mediaItems: [{ type: "image", asset: { url: "https://cdn.example.com/img.jpg", width: 800, height: 600 } as FeedPostMediaItemAsset } as FeedPostMediaItem],
    link: { url: "https://example.com/product", title: "Product" } as FeedPostLink
  };
  const result: CreateFeedPostResponse1 = await createFeedPost(tenantId, feedParams, "broadcast-987", true, false, true);
  console.log(result);
}
run();
[inline-code-end]