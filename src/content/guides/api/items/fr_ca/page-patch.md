[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité de mettre à jour une seule `Page`. Les commentaires correspondants seront mis à jour.

[inline-code-attrs-start title = 'Exemple cURL de Mise à Jour de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Mise à Jour de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Mise à Jour de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Included on failure. **/
    reason?: string
    user?: Page; // We return the complete updated page on success.
}
[inline-code-end]

#### Note

Certains paramètres dans l'objet Page sont automatiquement mis à jour. Ce sont les attributs de comptage et de titre. Les comptages ne peuvent pas être mis à jour
via l'API car ce sont des valeurs calculées. Le `title` de la page peut être défini via l'API, mais serait écrasé si le widget de commentaires est utilisé sur
une page avec le même `urlId` et un titre de page différent.
