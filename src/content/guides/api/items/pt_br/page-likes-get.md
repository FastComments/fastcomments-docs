[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Retorna o número de curtidas em uma página e se o usuário atual a curtiu. Páginas que ainda não existem retornam um `likeCount` de `0`.

Passe `includeUserInfo=public` para também obter os usuários que curtiram a página, mais recentes primeiro. Usuários com atividade de perfil privado e usuários anônimos são excluídos, e até 100 curtidas são pesquisadas. Apenas `public` é aceito aqui; `include-private` retorna um `422` e está disponível a partir de [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) com uma chave de API. Em páginas que exigem SSO, `includeUserInfo` retorna um `403` para usuários que não podem visualizar a página.

[inline-code-attrs-start title = 'Exemplo cURL de Curtidas de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Curtidas de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Curtidas de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
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