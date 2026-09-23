req
tenantId
afterId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| afterId | string | Nie |  |
| limit | number | Nie |  |
| tags | Array<string> | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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