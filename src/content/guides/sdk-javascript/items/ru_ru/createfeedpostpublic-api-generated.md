## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createFeedPostParams | CreateFeedPostParams | Да |  |
| broadcastId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_987654321";
const asset: FeedPostMediaItemAsset = { url: "https://cdn.fastcomments.com/uploads/team-photo.jpg", mimeType: "image/jpeg", sizeBytes: 324512 };
const mediaItem: FeedPostMediaItem = { type: "image", assets: [asset], caption: "Team launch day" };
const link: FeedPostLink = { url: "https://www.example.com/blog/product-update-march-2026", title: "Product update — March 2026" };
const createFeedPostParams: CreateFeedPostParams = {
  title: "Product update — March 2026",
  content: "<p>We shipped performance improvements and two new integrations.</p>",
  media: [mediaItem],
  link,
  visibility: "public",
  tags: ["product","release","march-2026"],
  customConfig: { allowComments: true, requireTOS: false }
};
const broadcastId: string = "broadcast_2026_03_25_live";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIxMjM0NSIsImlhdCI6MTY5MDI0MDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
[inline-code-end]