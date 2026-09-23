[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Enregistre un vote sur un sondage.

Un électeur ne peut avoir qu'un seul vote par sondage. L'appeler à nouveau pour le même électeur déplace son vote vers la nouvelle option plutôt que d'en ajouter un second, et voter pour l'option qu'il a déjà choisie ne fait rien.

La réponse inclut le sondage, vous obtenez ainsi les comptes mis à jour sans une seconde requête.

[inline-code-attrs-start title = 'Exemple cURL de création PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Exemple cURL de création anonyme PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de requête de création PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** L'un des champs userId ou anonUserId est requis. **/
    userId?: string
    anonUserId?: string
    /** L'IP de l'utilisateur final, utilisée pour la limitation de taux anonyme. Par défaut, l'IP de l'appelant. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de réponse de création PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Inclus en cas d'échec. **/
    reason?: string
    pollVote: PollVote
    /** Le sondage avec ses comptes mis à jour. **/
    poll: CommentPoll
}
[inline-code-end]

### Votes anonymes

Utilisez `anonUserId` au lieu de `userId` pour enregistrer un vote pour une personne qui n'est pas connectée. Cet identifiant n'a pas besoin de correspondre à un utilisateur quelque part – il identifie simplement la session, de sorte que la même personne ne soit pas comptée deux fois.

Le vote anonyme doit être activé pour votre site. Si le vote est limité aux utilisateurs connectés, un vote avec uniquement un `anonUserId` échoue avec `poll-login-required`.

Les votes anonymes sont également limités en fréquence par IP et par sondage, afin d'empêcher une personne de gonfler un sondage en réinitialisant sa session. Envoyez l'`ip` de l'utilisateur final afin que la limite s'applique à lui plutôt qu'à votre serveur.

### Autres notes

- Un `userId` doit correspondre à un utilisateur qui existe sur votre site. Les votes pour un utilisateur appartenant à un autre site sont rejetés.
- Voter sur un sondage fermé échoue avec `poll-closed`.
- Cette API met à jour les comptes du sondage et les pousse en direct vers les widgets connectés.

---