[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Curte uma página como o usuário atual. Cada usuário pode curtir uma página uma vez: curtir novamente tem sucesso com o código `already-liked` e não altera a contagem.

A página é criada se ainda não existir. Passe `title` para definir ou atualizar o título da página.

[inline-code-attrs-start title = 'Exemplo cURL de Curtir Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Curtir Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Define o título da página. **/
    title?: string
    /** JSON codificado em URI do seu objeto SSO. Omitir para usuários anônimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Curtir Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' quando o usuário já curtiu a página. Caso contrário, incluído em falhas. **/
    code?: 'already-liked' | string
    /** Incluído em falhas. **/
    reason?: string
}
[inline-code-end]

---