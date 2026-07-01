req
tenantId
afterId

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| afterId | string | 아니오 |  |
| limit | number | 아니오 |  |
| tags | Array<string> | 아니오 |  |

## 응답

반환: [`GetFeedPostsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse1.ts)

## 예제

[inline-code-attrs-start title = 'getFeedPosts 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const afterId: string = 'post_9876';
  const limit: number = 20;
  const tags: string[] = ['news', 'sports'];

  const feedResult: GetFeedPostsResponse1 = await getFeedPosts(tenantId, afterId, limit, tags);
})();
[inline-code-end]