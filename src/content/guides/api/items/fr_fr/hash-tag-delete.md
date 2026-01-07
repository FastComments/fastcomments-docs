[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Cette route permet la suppression d'un `HashTag` utilisateur par le tag fourni.

Notez qu'à moins que la création automatique de `HashTag` ne soit désactivée, les hashtags peuvent être recréés par un utilisateur fournissant le hashtag lors de la rédaction d'un commentaire.

[inline-code-attrs-start title = 'Exemple cURL de Suppression de HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Suppression de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Suppression de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
