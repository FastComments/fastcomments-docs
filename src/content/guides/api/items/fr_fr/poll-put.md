[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Attache un sondage à un commentaire existant, ou définit l'état complet du sondage qu'il possède déjà.

Le corps est le sondage complet, et les options que vous envoyez deviennent les options du sondage, dans cet ordre. Chaque option
est associée à son `id` :

- Une option envoyée avec le `id` d'une option existante conserve cette option et ses votes. Son libellé et sa position
  sont mis à jour selon ce que vous avez envoyé.
- Une option envoyée sans `id` est ajoutée, sans votes.
- Une option existante que vous omettez est supprimée, ainsi que les votes qui y ont été attribués. `totalVotes` diminue du même
  montant.

Ainsi, pour ajouter une option, envoyez les options actuelles avec leurs ids plus la nouvelle sans id. Pour en supprimer une,
envoyez la liste sans celle‑ci. Les ids d'option se trouvent dans le sondage retourné par `GET /api/v1/polls/:commentId`.

Envoyer aucun id ne remplace toutes les options et supprime tous les votes déjà enregistrés sur le sondage. Si le sondage a
des votes, cela nécessite `replaceVotes=true`, et sans cela l'API répond avec `replace-votes-required`.

Les autres champs sont également remplacés : omettre `closesAt`, `privacy` ou `requireVoteToSeeResults` les réinitialise à
leur valeur par défaut. Pour modifier un seul champ et laisser les autres intacts, utilisez `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Exemple cURL de mise à jour du sondage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Requis pour ne conserver aucun des identifiants d'option existants lorsque le sondage a des votes, car cela les supprime tous. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** L'identifiant d'une option existante, pour la conserver ainsi que ses votes. Omettre pour ajouter une nouvelle option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** La liste complète et ordonnée. Les options existantes omises sont supprimées avec leurs votes. **/
    options: PollPutOption[]
    /** Doit être dans le futur lorsque le commentaire n'a pas encore de sondage. Omettre pour un sondage qui reste ouvert. **/
    closesAt?: string | null
    /** 0 anonyme (par défaut), 1 administrateurs et modérateurs, 2 tout le monde. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Inclus en cas d'échec. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Inclus en cas d'échec. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Autres notes

- Un `id` qui n'est pas présent dans le sondage, ou le même `id` fourni deux fois, échoue avec `poll-invalid`. Un commentaire sans
  sondage n'a pas encore d'identifiants d'option, donc chaque option envoyée doit omettre `id`.
- La confidentialité du sondage peut être restreinte mais pas élargie une fois qu'il a des votes.
- Cette API respecte les paramètres de votre site. Si les sondages ne sont pas activés pour le site ou la page, elle échoue avec
  `polls-disabled`.
- Un commentaire verrouillé ne peut pas voir son sondage modifié, et échoue avec `locked`.
- Les widgets connectés sont mis à jour en temps réel, de sorte que les spectateurs voient le nouveau sondage sans recharger.