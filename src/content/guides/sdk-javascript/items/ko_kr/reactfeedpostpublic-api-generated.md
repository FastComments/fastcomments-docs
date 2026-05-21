## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| reactBodyParams | ReactBodyParams | 예 |  |
| isUndo | boolean | 아니요 |  |
| broadcastId | string | 아니요 |  |
| urlId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'reactFeedPostPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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