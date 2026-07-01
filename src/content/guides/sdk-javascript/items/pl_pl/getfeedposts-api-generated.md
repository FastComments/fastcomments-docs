req  
tenantId  
afterId  

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| afterId | string | Nie |  |
| limit | number | Nie |  |
| tags | Array<string> | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'getFeedPosts Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
(async () => {  
  const tenantId: string = 'tenant_12345';  
  const afterId: string = 'post_9876';  
  const limit: number = 20;  
  const tags: string[] = ['news', 'sports'];  

  const feedResult: GetFeedPostsResponse1 = await getFeedPosts(tenantId, afterId, limit, tags);  
})();  
[inline-code-end]