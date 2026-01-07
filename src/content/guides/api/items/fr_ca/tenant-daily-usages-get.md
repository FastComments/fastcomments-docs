[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Cette route permet de rechercher l'utilisation d'un locataire par année, mois et jour. Jusqu'à 365 objets peuvent être retournés, et le coût est de 1 crédit API par 10 objets.

Les objets de réponse sont triés par la date de création (les plus anciens en premier).

[inline-code-attrs-start title = 'Exemple cURL de Recherche de TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Based on UTC. **/
    yearNumber?: number
    /** Zero-based. Based on UTC. **/
    monthNumber?: number
    /** One-based. Based on UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]
