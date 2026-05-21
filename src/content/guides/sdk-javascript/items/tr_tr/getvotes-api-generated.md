## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |

## Yanıt

Döndürür: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getVotes Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42c-eu';
const urlId: string = 'article-7f9b';
const includeMetadata: boolean | undefined = true;
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]

---