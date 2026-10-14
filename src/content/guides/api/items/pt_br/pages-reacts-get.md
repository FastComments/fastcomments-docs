[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Retorna a contagem para cada reação em uma página, e opcionalmente os usuários que reagiram, como uma entrada por reação com seu ID de reação, mais recente primeiro. Esta é a versão do lado do servidor de [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` controla quais usuários são retornados:

- `public` omite usuários que tornaram a atividade de seu perfil privada.
- `include-private` os inclui. Use isso apenas quando a lista não for mostrada ao público.

Usuários anônimos nunca são incluídos, e até 100 reações são pesquisadas. Páginas que não existem retornam nenhum `counts` e uma lista `users` vazia.

[inline-code-attrs-start title = 'Exemplo cURL de Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omitir para obter apenas as contagens. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Contagem por ID de reação, por exemplo {"heart": 12, "laugh": 3}. Não definido quando a página não tem reações. **/
    counts?: Record<string, number>
    /** Uma entrada por reação, mais recente primeiro. Definido apenas quando includeUserInfo é passado. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** O ID da reação, por exemplo "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Dica Útil

Lembre-se de codificar em URI valores como o `urlId`.

---