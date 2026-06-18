## Параметры

| Name | Type | Required | Description |
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
(async () => {
  const tenantId: string = "tenant_9f8b7c";
  const media: FeedPostMediaItem[] = [{ type: "image", assets: [{ url: "https://cdn.example.com/roadmap.jpg", mimeType: "image/jpeg" }] }];
  const links: FeedPostLink[] = [{ url: "https://company.example.com/roadmap", title: "Full roadmap" }];
  const createFeedPostParams: CreateFeedPostParams = {
    title: "Weekly Product Roadmap Update",
    body: "This week we shipped enhancements to search relevance and fixed top customer bugs.",
    authorId: "user_8321",
    media,
    links,
    visibility: "public"
  };
  const broadcastId: string = "broadcast_2026_06_15";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload";
  const response: CreateFeedPostPublic200Response = await createFeedPostPublic(tenantId, createFeedPostParams, broadcastId, sso);
  console.log(response);
})();
[inline-code-end]