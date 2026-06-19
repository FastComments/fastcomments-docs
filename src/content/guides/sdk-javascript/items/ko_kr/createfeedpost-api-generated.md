## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createFeedPostParams | CreateFeedPostParams | 예 |  |
| broadcastId | string | 아니오 |  |
| isLive | boolean | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| skipDupCheck | boolean | 아니오 |  |

## 응답

반환: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostsResponse.ts)

## 예제

[inline-code-attrs-start title = 'createFeedPost 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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