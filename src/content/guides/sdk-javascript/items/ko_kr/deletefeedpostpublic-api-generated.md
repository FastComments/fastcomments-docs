## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## 응답

반환: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteFeedPostPublic 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const postId: string = "post_987654321";
  const broadcastId: string = "broadcast_2024Q1";
  const sso: string = "sso_4fa3b9c2";

  const response1: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId);
  const response2: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId);
  const response3: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
})();
[inline-code-end]