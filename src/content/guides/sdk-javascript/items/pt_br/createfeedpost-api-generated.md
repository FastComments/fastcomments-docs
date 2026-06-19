## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createFeedPostParams | CreateFeedPostParams | Sim |  |
| broadcastId | string | Não |  |
| isLive | boolean | Não |  |
| doSpamCheck | boolean | Não |  |
| skipDupCheck | boolean | Não |  |

## Resposta

Retorna: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createFeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_82b4a1";
const asset: FeedPostMediaItemAsset = { url: "https://cdn.company.com/uploads/launch.jpg", mimeType: "image/jpeg", width: 1200, height: 800 };
const mediaItem: FeedPostMediaItem = { type: "image", caption: "Launch screenshot", assets: [asset] };
const link: FeedPostLink = { url: "https://company.com/blog/launch-offline-sync", title: "Offline sync release notes" };
const createFeedPostParams: CreateFeedPostParams = { message: "Offline sync is now available — try it on mobile for faster access.", media: [mediaItem], links: [link], visibility: "public" };
const broadcastId: string = "brdcst_20260619_01";
const isLive: boolean = false;
const doSpamCheck: boolean = true;
const skipDupCheck: boolean = false;
const result: CreateFeedPostsResponse = await createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck);
[inline-code-end]

---