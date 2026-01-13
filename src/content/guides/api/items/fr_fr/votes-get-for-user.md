[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Permet de récupérer les votes laissés par un utilisateur sur un `urlId` donné. Prend un `userId` qui peut être n'importe quel utilisateur FastComments.com ou `SSO User`.

Ceci est utile si vous voulez montrer si un utilisateur a voté sur un commentaire. Lors de la récupération des commentaires, appelez simplement cette API en même temps pour l'utilisateur avec le
même `urlId`.

Si vous utilisez le vote anonyme, vous voudrez passer `anonUserId` à la place.

[inline-code-attrs-start title = 'Exemple cURL de Votes pour Utilisateur'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Exemple cURL de Votes pour Utilisateur Anonyme'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Notez que les votes anonymes apparaîtront dans la liste `appliedAuthorizedVotes`. Ils sont considérés comme autorisés puisqu'ils ont été créés via l'API avec une clé API.

[inline-code-attrs-start title = 'Structure de Requête de Votes pour Utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Votes pour Utilisateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
