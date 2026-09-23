[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Retorna a contagem para cada reação em uma página e quais reações o usuário atual adicionou.

[inline-code-attrs-start title = 'Exemplo cURL de Reações de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Reações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON codificado em URI do seu objeto SSO. Omitir para usuários anônimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Reações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: string
    /** Incluído em caso de falha. **/
    reason?: string
    /** Contagem por id de reação, por exemplo {"heart": 12, "laugh": 3}. Não definido quando a página não tem reações. **/
    counts?: Record<string, number>
    /** Os ids de reação que o usuário que fez a requisição adicionou. Não definido quando ele não adicionou nenhum. **/
    reactedIds?: string[]
}
[inline-code-end]

---