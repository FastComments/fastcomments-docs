## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createFeedPostParams | CreateFeedPostParams | Да |  |
| broadcastId | string | Не |  |
| isLive | boolean | Не |  |
| doSpamCheck | boolean | Не |  |
| skipDupCheck | boolean | Не |  |

## Одговор

Враћа: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPost200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createFeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2b1c';
const createFeedPostParams: CreateFeedPostParams = {
  content: 'Launching our summer collection today — check it out!',
  authorId: 'user_879',
  media: [
    {
      type: 'image',
      assets: [
        { url: 'https://cdn.myshop.com/uploads/summer-look.jpg', width: 1200, height: 800 } as FeedPostMediaItemAsset
      ]
    } as FeedPostMediaItem
  ],
  links: [
    { url: 'https://myshop.com/new-arrival', title: 'Summer Collection' } as FeedPostLink
  ],
  allowComments: true
};
const broadcastId: string = 'broadcast-2026-06-15-001';
const isLive: boolean = false;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const response: CreateFeedPost200Response = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]

---