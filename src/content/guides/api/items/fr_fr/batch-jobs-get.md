[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Cette route renvoie les jobs batch du locataire pour chaque type, du plus récent au plus ancien, 100 par page. Utilisez `type` et `status` pour affiner la liste, et `skip` pour paginer.

[inline-code-attrs-start title = 'Exemple cURL de la liste BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Uniquement les jobs de ce type, par exemple migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Nombre de jobs à ignorer, pour la pagination. Un entier non négatif. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Inclus en cas d'échec. **/
    reason?: string
    /** Du plus récent au plus ancien, au maximum 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]