req
tenantId
afterId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | number | No |  |
| tags | Array<string> | No |  |

## Odgovor

Vrati: [`GetFeedPostsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const afterId: string = 'post_9876';
  const limit: number = 20;
  const tags: string[] = ['news', 'sports'];

  const feedResult: GetFeedPostsResponse1 = await getFeedPosts(tenantId, afterId, limit, tags);
})();
[inline-code-end]

---