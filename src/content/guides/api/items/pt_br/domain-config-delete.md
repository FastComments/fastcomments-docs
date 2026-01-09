[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a remoção de um único `DomainConfig` por id.

- Observação: Remover um `DomainConfig` irá desautorizar esse domínio de usar o FastComments.
- Observação: Re-adicionar um domínio pela interface do usuário (UI) recriará o objeto (com apenas `domain` preenchido).

[inline-code-attrs-start title = 'Exemplo cURL de remoção de DomainConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição de remoção de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta de remoção de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---