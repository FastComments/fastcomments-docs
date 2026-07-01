req
tenantId
afterId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| limit | number | Όχι |  |
| tags | Array<string> | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetFeedPostsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getFeedPosts Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const afterId: string = 'post_9876';
  const limit: number = 20;
  const tags: string[] = ['news', 'sports'];

  const feedResult: GetFeedPostsResponse1 = await getFeedPosts(tenantId, afterId, limit, tags);
})();
[inline-code-end]