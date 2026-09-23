[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Remove uma das reações do usuário atual de uma página. Se o usuário não adicionou essa reação, a solicitação tem sucesso com o código `no-react` e não altera a contagem.

[inline-code-attrs-start title = 'Exemplo cURL de Exclusão de Reação de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Solicitação de Exclusão de Reação de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Exclusão de Reação de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' when the user had not added this reaction. Otherwise included on failure. **/
    code?: 'no-react' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]