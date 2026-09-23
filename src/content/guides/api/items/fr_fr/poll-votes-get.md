[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Liste les votes individuels derrière les comptes d'un sondage, du plus ancien au plus récent. Un crédit par 100 votes retournés.

Un sondage appartient à un commentaire, donc les votes sont lus un sondage à la fois et `commentId` est requis. Affinez davantage avec `voterId` pour vérifier comment une personne a voté, ou avec `optionId` pour lister tous ceux qui ont choisi une option donnée.

Au maximum 1000 votes sont retournés par appel. Utilisez `skip` pour paginer davantage.

Le paramètre `privacy` du sondage est respecté : les votes d'un sondage anonyme ne peuvent pas être lus, et la requête échoue avec `poll-anonymous`. Consultez la structure `PollVote` pour plus de détails.

[inline-code-attrs-start title = 'Exemple cURL de récupération des votes de sondage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête GET PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse GET PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Comptage des votes par option

Vous n'avez pas besoin d'additionner ces votes pour obtenir les résultats – le sondage possède ses propres comptes. Lisez le sondage avec `GET /api/v1/polls/:commentId` à la place, et utilisez cette API lorsque vous avez besoin de savoir qui a voté.

### Tous les sondages sur une page

Il n'existe pas de liste de votes à l'échelle de la page. Pour faire un rapport sur une page entière, récupérez ses commentaires avec `GET /api/v1/comments`, qui renvoie le sondage de chaque commentaire ainsi que ses comptes, puis lisez les votes des sondages qui vous intéressent.