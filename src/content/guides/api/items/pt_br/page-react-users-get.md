[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Retorna os nomes dos usuários que adicionaram uma reação a uma página, ordenados alfabeticamente. Até 100 reações são pesquisadas, e usuários anônimos não são incluídos. Em páginas que exigem SSO, usuários que não podem visualizar a página recebem um `403` com o código `sso-required` ou `page-forbidden`.

Para também obter avatares e IDs de usuário, use [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) com `includeUserInfo=public`.

[inline-code-attrs-start title = 'Exemplo cURL de Usuários de Reação de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Usuários de Reação de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Usuários de Reação de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---