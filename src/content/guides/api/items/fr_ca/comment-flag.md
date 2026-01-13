[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de signaler un commentaire pour un utilisateur spécifique.

Notes :

- Cet appel doit toujours être effectué dans le contexte d'un utilisateur. L'utilisateur peut être un utilisateur FastComments.com, un utilisateur SSO ou un utilisateur de locataire.
- Si un seuil de signalement pour masquer est défini, le commentaire sera automatiquement masqué en direct après avoir été signalé le nombre de fois défini.
- Après qu'il soit automatiquement désapprouvé (masqué) - le commentaire ne peut être réapprouvé que par un administrateur ou un modérateur. Annuler le signalement ne réapprouvera pas le commentaire.

[inline-code-attrs-start title = 'Exemple cURL de Signalement de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Pour le signalement anonyme, nous devons spécifier un `anonUserId`. Cela peut être un ID qui représente la session anonyme, ou un UUID aléatoire.
Cela nous permet de prendre en charge le signalement et l'annulation du signalement des commentaires même si un utilisateur n'est pas connecté. De cette façon, le commentaire peut être marqué comme
signalé lorsque les commentaires sont récupérés avec le même `anonUserId`.

[inline-code-attrs-start title = 'Exemple cURL de Signalement de Commentaire Anonyme'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structure de Requête de Signalement de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Signalement de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]
