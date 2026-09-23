req
tenantId
afterId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| limit | number | Nee |  |
| tags | Array<string> | Nee |  |

## Respons

Retourneert: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPosts Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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