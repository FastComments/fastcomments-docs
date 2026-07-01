## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createFeedPostParams | CreateFeedPostParams | Ja |  |
| broadcastId | string | Nee |  |
| isLive | boolean | Nee |  |
| doSpamCheck | boolean | Nee |  |
| skipDupCheck | boolean | Nee |  |

## Respons

Retour: [`CreateFeedPostResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createFeedPost Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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