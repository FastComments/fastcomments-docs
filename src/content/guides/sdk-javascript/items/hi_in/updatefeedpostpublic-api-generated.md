## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postId | string | हाँ |  |
| updateFeedPostParams | UpdateFeedPostParams | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateFeedPostPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// पैरामीटर परिभाषित करें
const tenantId: string = "tenant_12345";
const postId: string = "post_98765";

const mediaAsset: FeedPostMediaItemAsset = {
  url: "https://cdn.example.com/image.jpg",
  width: 800,
  height: 600,
  mimeType: "image/jpeg"
};

const mediaItem: FeedPostMediaItem = {
  type: "image",
  asset: mediaAsset,
  caption: "A beautiful scenery"
};

const link: FeedPostLink = {
  url: "https://example.com/article",
  title: "Interesting Article",
  description: "An article about TypeScript best practices."
};

const updateParams: UpdateFeedPostParams = {
  content: "Updated post content with new media and link.",
  media: [mediaItem],
  link: link,
  isPublic: true
};

const broadcastId: string = "broadcast_001";
const sso: string = "sso_token_abc123";

const response: CreateFeedPostResponse = await updateFeedPostPublic(
  tenantId,
  postId,
  updateParams,
  broadcastId,
  sso
);
[inline-code-end]