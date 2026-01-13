[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de bloquer un utilisateur qui a écrit un commentaire donné. Il prend en charge le blocage des commentaires écrits par les utilisateurs FastComments.com, les utilisateurs SSO et les utilisateurs de locataire.

Il prend en charge un paramètre de corps `commentIdsToCheck` pour vérifier si d'autres commentaires potentiellement visibles sur le client doivent être bloqués/débloqués après cette action.

Notes :

- Cet appel doit toujours être effectué dans le contexte d'un utilisateur. L'utilisateur peut être un utilisateur FastComments.com, un utilisateur SSO ou un utilisateur de locataire.
- Le `userId` dans la requête est l'utilisateur qui *effectue le blocage*. Par exemple : `Utilisateur A` veut bloquer `Utilisateur B`. Passez `userId=Utilisateur A` et l'id du commentaire que `Utilisateur B` a écrit.
- Les commentaires complètement anonymes (pas d'id utilisateur, pas d'email) ne peuvent pas être bloqués et une erreur sera retournée.

[inline-code-attrs-start title = 'Exemple cURL de Blocage de Commentaire'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Pour le blocage anonyme, nous devons spécifier un `anonUserId`. Cela peut être un ID qui représente la session anonyme, ou un UUID aléatoire.
Cela nous permet de prendre en charge le blocage des commentaires même si un utilisateur n'est pas connecté en récupérant les commentaires avec le même `anonUserId`.

[inline-code-attrs-start title = 'Exemple cURL de Blocage de Commentaire Anonyme'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Blocage de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Blocage de Commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
