## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |

## Odpowiedź

Zwraca: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42c-eu';
const urlId: string = 'article-7f9b';
const includeMetadata: boolean | undefined = true;
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]

---