[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Vous pouvez actuellement uniquement récupérer toutes les pages (ou une seule page via `/by-url-id`) associées à votre compte. Si vous souhaitez une recherche plus fine, [contactez-nous](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Exemple cURL de Pages'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Conseil Utile

L'API `Comment` nécessite un `urlId`. Vous pouvez d'abord appeler l'API `Pages`, pour voir à quoi ressemblent les valeurs `urlId` disponibles pour vous.
