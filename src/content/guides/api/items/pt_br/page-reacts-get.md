[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Retorna a contagem para cada reação em uma página e quais reações o usuário atual adicionou.

Passe `includeUserInfo=public` para também obter os usuários que reagiram, como uma entrada por reação com seu ID de reação, mais recentes primeiro. Usuários com atividade de perfil privado e usuários anônimos são excluídos, e até 100 reações são pesquisadas. Apenas `public` é aceito aqui; `include-private` retorna um `422` e está disponível em [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) com uma chave de API. Em páginas que exigem SSO, `includeUserInfo` retorna um `403` para usuários que não podem visualizar a página.

[inline-code-attrs-start title = 'Exemplo cURL de Reações de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Reações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who reacted. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Reações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** The reaction ids the user making the request has added. Not set when they have added none. **/
    reactedIds?: string[]
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]