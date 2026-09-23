## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Ответ

Возвращает: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostResponse.ts)

## Пример

[inline-code-attrs-start title = 'updateFeedPostPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Определить параметры
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