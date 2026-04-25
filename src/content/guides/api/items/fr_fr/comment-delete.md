[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ce point de terminaison d'API permet de supprimer un commentaire.

Remarques:

- Cette API peut mettre à jour le widget de commentaires "en direct" si souhaité (cela augmente `creditsCost` de `1` à `2`).
- Cette API supprimera tous les commentaires enfants.
- Si le commentaire ciblé est verrouillé (`isLocked: true`), la requête est rejetée avec `code: 'locked'`. Déverrouillez d'abord le commentaire, puis supprimez-le.

[inline-code-attrs-start title = 'Exemple cURL de suppression de commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structure de la requête DELETE de commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** L'utilisateur effectuant la mise à jour. Peut être utilisé, si souhaité, pour vérifier qu'il peut supprimer le commentaire.  **/
    contextUserId?: string
	/** Indique si le commentaire doit être supprimé "en direct" pour les utilisateurs visualisant des instances du widget de commentaires avec le même urlId. REMARQUE : double le coût en crédits de 1 à 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse DELETE de commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Inclus en cas d'échec. **/
    reason?: string
}
[inline-code-end]

---