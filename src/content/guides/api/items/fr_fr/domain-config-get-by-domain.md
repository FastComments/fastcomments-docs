[api-resource-header-start name = 'DomainConfig'; route = 'GET /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Les DomainConfigs individuels peuvent être récupérés par leur `domain` correspondant.

[inline-code-attrs-start title = 'Exemple cURL de Config de Domaine par Domaine'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Config de Domaine par Domaine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigsByDomainRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Config de Domaine par Domaine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-domain' | 'update-would-create-duplicate' | 'domain-does-not-exist'
    /** Included on failure. **/
    reason?: string
    configuration?: DomainConfig | null
}
[inline-code-end]
