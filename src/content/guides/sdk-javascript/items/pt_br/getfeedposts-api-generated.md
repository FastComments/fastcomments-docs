---
req
tenantId
afterId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| limit | number | Não |  |
| tags | Array<string> | Não |  |

## Resposta

Retorna: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---