---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Да |  |
| reactBodyParams | ReactBodyParams | Да |  |
| isUndo | boolean | Не |  |
| broadcastId | string | Не |  |
| urlId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'reactFeedPostPublic пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "global-markets";
const postId: string = "8e2c3f9a-4b6d-4f1a-9c2d-e8a1b2c3d4e5";
const reactBodyParams: ReactBodyParams = { reactionType: "like", clientApp: "web-ui", timestamp: new Date().toISOString() };
const isUndo: boolean = false;
const broadcastId: string = "broadcast-2026-05-20";
const urlId: string = "feed-post-8e2c";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";

const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, urlId, sso);
[inline-code-end]

---