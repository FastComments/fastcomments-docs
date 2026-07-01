req
tenantId
afterId

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | number | 否 |  |
| tags | Array<string> | 否 |  |

## 响应

返回：[`GetFeedPostsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getFeedPosts 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const afterId: string = 'post_9876';
  const limit: number = 20;
  const tags: string[] = ['news', 'sports'];

  const feedResult: GetFeedPostsResponse1 = await getFeedPosts(tenantId, afterId, limit, tags);
})();
[inline-code-end]