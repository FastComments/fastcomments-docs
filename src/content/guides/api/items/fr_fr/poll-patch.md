[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Modifie un sondage sans perturber ses votes. Utilisez ceci pour corriger une faute de frappe dans la question ou une option, pour fermer ou
rouvrir le sondage, ou pour changer qui peut voir qui a voté.

Les options sont identifiées par leur `id`, et un `PATCH` renomme celles que vous indiquez. Pour ajouter, supprimer ou réorganiser
les options, envoyez la liste complète des options à `PUT /api/v1/polls/:commentId` : les options que vous envoyez avec leurs ids conservent
leurs votes également.

Chaque champ est optionnel, mais au moins un doit être fourni.

[inline-code-attrs-start title = 'Exemple cURL de mise à jour du sondage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Exemple cURL de fermeture immédiate d\'un sondage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête de mise à jour du sondage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse de mise à jour du sondage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Autres notes

- Nommer un id d'option qui n'est pas présent dans le sondage échoue avec `poll-invalid` plutôt que de ne rien faire silencieusement.
- Les libellés doivent rester uniques dans le sondage, en comptant les options que vous ne modifiez pas.
- Contrairement à la création d'un sondage, `closesAt` peut être dans le passé ici – c'est ainsi que vous fermez immédiatement un sondage.
- La confidentialité du sondage peut être restreinte mais pas élargie une fois qu'il possède des votes.
- Un commentaire verrouillé ne peut pas voir son sondage modifié, et échoue avec `locked`.

---