---
[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Remove o curtir do usuário atual de uma página. Se o usuário não curtiu a página, a requisição tem sucesso com o código `not-liked` e não altera a contagem.

[inline-code-attrs-start title = 'Exemplo cURL de Descurtir Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Descurtir Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** JSON codificado em URI do seu objeto SSO. Omitir para usuários anônimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Descurtir Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' quando o usuário não curtiu a página. Caso contrário incluído em falhas. **/
    code?: 'not-liked' | string
    /** Incluído em falhas. **/
    reason?: string
}
[inline-code-end]

---