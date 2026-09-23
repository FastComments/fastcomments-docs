## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| updateFeedPostParams | UpdateFeedPostParams | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## 예시

[inline-code-attrs-start title = 'updateFeedPostPublic 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 매개변수 정의
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

---