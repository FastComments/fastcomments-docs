[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de créer des pages.

Un cas d'utilisation courant est le contrôle d'accès.

Notes :

- Si vous avez commenté sur un fil de commentaires, ou appelé l'API pour créer un `Comment`, vous avez déjà créé un objet `Page` ! Vous pouvez essayer de le récupérer via
  la route `Page` `/by-url-id`, en passant le même `urlId` passé au widget de commentaires.
- La structure `Page` contient certaines valeurs **calculées**.
  Actuellement, ce sont `commentCount` et `rootCommentCount`.
  Elles sont remplies automatiquement et ne peuvent pas être définies par l'API. Tenter de le faire provoquera une erreur de l'API.

[inline-code-attrs-start title = 'Exemple cURL de POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
