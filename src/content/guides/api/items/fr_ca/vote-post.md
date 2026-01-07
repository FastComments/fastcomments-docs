 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Cette route fournit la capacité d'ajouter un seul `Vote` autorisé. Les votes peuvent être `up` (+1) ou `down` (-1).

[inline-code-attrs-start title = 'Exemple cURL de Création de Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Exemple cURL de Création de Vote Anonyme'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Création de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Création de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Création de Votes Anonymes

Les votes anonymes peuvent être créés en définissant `anonUserId` dans les paramètres de requête au lieu de `userId`.

Cet id n'a pas à correspondre à un objet utilisateur quelque part (d'où anonyme). C'est simplement un identifiant
pour la session, afin que vous puissiez récupérer les votes à nouveau dans la même session, pour vérifier si un commentaire a
été voté.

Si vous n'avez pas de concept de "sessions anonymes" comme FastComments - vous pouvez simplement
définir ceci sur un ID aléatoire, comme un UUID (bien que nous apprécions les identifiants plus petits pour économiser de l'espace).

### Autres Notes

- Cette API obéit aux paramètres au niveau du locataire. Par exemple, si vous désactivez le vote pour une page donnée, et que vous tentez de créer un vote via l'API, cela échouera avec le code d'erreur `voting-disabled`.
- Cette API est en direct par défaut.
- Cette API mettra à jour les `votes` du `Comment` correspondant.
