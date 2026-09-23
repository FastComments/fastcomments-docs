요청
tenantId
afterId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | number | No |  |
| tags | Array<string> | No |  |

## 응답

반환: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getFeedPosts 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadFeed() {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const afterId: string = 'post_987654321';
  const limit: number = 25;
  const tags: string[] = ['technology', 'innovation'];
  const response: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
  console.log(response);
}
loadFeed();
[inline-code-end]

---