[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Retorna o número de curtidas em uma página e, opcionalmente, os usuários que a curtiram, mais recentes primeiro. Esta é a versão do lado do servidor de [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` controla quais usuários são retornados:

- `public` omite usuários que tornaram a atividade de seu perfil privada.
- `include-private` inclui‑os. Use isso somente quando a lista não for exibida ao público.

Usuários anônimos nunca são incluídos, e até 100 curtidas são pesquisadas. Páginas que não existem retornam um `likeCount` de `0` e uma lista `users` vazia.

[inline-code-attrs-start title = 'Exemplo cURL de Curtidas de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Curtidas de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Curtidas de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Dica Útil

Lembre‑se de codificar em URI valores como o `urlId`.