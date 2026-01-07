[api-resource-header-start name = 'Page'; route = 'DELETE /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Cette route permet la suppression d'une seule page par id.

Notez qu'interagir avec le widget de commentaires pour une page avec le même `urlId` recréera simplement la `Page` de manière transparente.

[inline-code-attrs-start title = 'Exemple cURL de Suppression de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pages/some-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Suppression de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Suppression de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'page-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
