req
tenantId
afterId

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니요 |  |
| limit | number | 아니요 |  |
| tags | Array<string> | 아니요 |  |

## 응답

반환: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## 예제

[inline-code-attrs-start title = 'getFeedPosts 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const initialPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', undefined, 20, ['sports', 'local']);
const nextPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', 'post_abc123', 20, ['sports', 'local']);
[inline-code-end]