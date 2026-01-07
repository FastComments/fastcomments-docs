[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Les votes doivent être récupérés par `urlId`.

### Types de Votes

Il existe trois types de votes :

- Votes authentifiés, qui sont appliqués au commentaire correspondant. Vous pouvez les créer via cette API.
- Votes authentifiés, qui sont **en attente** de vérification, et donc ne sont pas encore appliqués au commentaire. Ceux-ci sont créés lorsqu'un utilisateur utilise le mécanisme *connexion pour voter* de FastComments.com.
- Votes anonymes, qui sont appliqués au commentaire correspondant. Ceux-ci sont créés avec les commentaires anonymes.

Ceux-ci sont retournés dans des listes séparées dans l'API pour réduire la confusion.

[inline-code-attrs-start title = 'Exemple cURL de Votes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Notes sur les Votes Anonymes

Notez que les votes anonymes créés via cette API apparaîtront dans la liste `appliedAuthorizedVotes`. Ils sont considérés comme autorisés puisqu'ils ont été créés via l'API avec une clé API.

La structure `appliedAnonymousVotes` est pour les votes créés sans email, clé API, etc.
