[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Lit le sondage attaché à un commentaire, avec ses comptes de votes actuels.

Les sondages sont également renvoyés avec le commentaire lui‑même par les API de commentaires, donc utilisez ceci lorsque vous ne voulez que les résultats et non le commentaire complet.

[inline-code-attrs-start title = 'Exemple cURL de récupération du sondage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de récupération du sondage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de récupération du sondage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Un commentaire qui n’a pas de sondage, un commentaire qui a été supprimé, et un identifiant de commentaire qui n’existe pas répondent tous de la même manière, avec `poll-not-found`.

---