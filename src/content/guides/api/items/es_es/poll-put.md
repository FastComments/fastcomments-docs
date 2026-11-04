[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Adjunta una encuesta a un comentario existente, o establece el estado completo de la encuesta que ya tiene.

El cuerpo es la encuesta completa, y las opciones que envíes se convierten en las opciones de la encuesta, en ese orden. Cada opción
se empareja por su `id`:

- Una opción enviada con el `id` de una opción existente conserva esa opción y sus votos. Su etiqueta y posición
  se actualizan a lo que enviaste.
- Una opción enviada sin `id` se añade, sin votos.
- Una opción existente que omitas se elimina, junto con los votos emitidos sobre ella. `totalVotes` disminuye en la misma
  cantidad.

Así que para añadir una opción, envía las opciones actuales con sus ids más la nueva sin id. Para eliminar una,
envía la lista sin ella. Los ids de las opciones están en la encuesta devuelta por `GET /api/v1/polls/:commentId`.

No enviar ningún id reemplaza todas las opciones y elimina todos los votos ya emitidos en la encuesta. Si la encuesta tiene
votos, esto requiere `replaceVotes=true`, y sin él la API responde con `replace-votes-required`.

Los demás campos también se reemplazan: omitir `closesAt`, `privacy` o `requireVoteToSeeResults` lo restablece a
su valor predeterminado. Para cambiar un solo campo y dejar el resto igual, usa `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Ejemplo de cURL para Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estructura de la solicitud Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Otras notas

- Un `id` que no esté en la encuesta, o el mismo `id` dado dos veces, falla con `poll-invalid`. Un comentario sin
  encuesta aún no tiene ids de opciones, por lo que cada opción enviada a él debe omitir `id`.
- La privacidad de la encuesta puede restringirse pero no ampliarse una vez que tiene votos.
- Esta API respeta la configuración de tu sitio. Si las encuestas no están habilitadas para el sitio o la página, falla con
  `polls-disabled`.
- Un comentario bloqueado no puede tener su encuesta modificada, y falla con `locked`.
- Los widgets conectados se actualizan en vivo, por lo que los espectadores ven la nueva encuesta sin recargar.