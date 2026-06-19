req
tenantId
afterId

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | number | Не |  |
| tags | Array<string> | Не |  |

## Отговор

Връща: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---