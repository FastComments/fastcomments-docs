[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Cette route fournit la suppression d'un `Tenant` **et toutes les données associées** (utilisateurs, commentaires, etc) par id.

Les restrictions suivantes existent concernant la suppression de locataires:

- Le locataire doit être le vôtre, ou un locataire en marque blanche que vous gérez.
- Le paramètre de requête `sure` doit être défini à `true`.

[inline-code-attrs-start title = 'Exemple cURL de Suppression de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Suppression de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Suppression de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
