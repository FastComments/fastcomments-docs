[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité d'annuler le signalement d'un commentaire pour un utilisateur spécifique.

Notes :

- Cet appel doit toujours être effectué dans le contexte d'un utilisateur. L'utilisateur peut être un utilisateur FastComments.com, un utilisateur SSO ou un utilisateur de locataire.
- Après qu'un commentaire soit automatiquement désapprouvé (masqué) - le commentaire ne peut être réapprouvé que par un administrateur ou un modérateur. Annuler le signalement ne réapprouvera pas le commentaire.

[inline-code-attrs-start title = 'Exemple cURL d’Annulation de Signalement de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Pour le signalement anonyme, nous devons spécifier un `anonUserId`. Cela peut être un ID qui représente la session anonyme, ou un UUID aléatoire.

[inline-code-attrs-start title = 'Exemple cURL de Signalement Anonyme de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structure de Requête d’Annulation de Signalement de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse d’Annulation de Signalement de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
