## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| broadcastId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteFeedPostPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f8a9b3c";
const postId: string = "post_a1b2c3d4";
const broadcastId: string = "broadcast_2026-03-25T10:00:00Z";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiaWF0IjoxNjE5MjM5MjAwfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

const resultWithOptional: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
const resultRequiredOnly: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId);
[inline-code-end]

---