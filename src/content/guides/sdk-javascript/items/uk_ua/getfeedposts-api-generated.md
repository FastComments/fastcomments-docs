req
tenantId
afterId

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| afterId | string | Ні |  |
| limit | number | Ні |  |
| tags | Array<string> | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---