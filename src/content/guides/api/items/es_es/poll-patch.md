[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Edita una encuesta sin alterar sus votos. Usa esto para corregir un error tipográfico en la pregunta o en una opción, para cerrar o reabrir la encuesta, o para cambiar quién puede ver quién votó.

Las opciones se identifican por su `id`, y un `PATCH` renombra las que indiques. Para agregar, eliminar o reordenar opciones, envía la lista completa de opciones a `PUT /api/v1/polls/:commentId`: las opciones que envíes con sus ids conservan también sus votos allí.

Cada campo es opcional, pero al menos uno debe proporcionarse.

[inline-code-attrs-start title = 'Ejemplo cURL de parche de encuesta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Ejemplo cURL de cerrar una encuesta ahora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de parche de encuesta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estructura de respuesta de parche de encuesta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Otras notas

- Nombrar un id de opción que no está en la encuesta falla con `poll-invalid` en lugar de no hacer nada silenciosamente.
- Las etiquetas deben permanecer únicas dentro de la encuesta, contando también las opciones que no estás cambiando.
- A diferencia de crear una encuesta, `closesAt` puede estar en el pasado aquí; así es como cierras una encuesta inmediatamente.
- La privacidad de la encuesta puede restringirse pero no ampliarse una vez que tiene votos.
- Un comentario bloqueado no puede tener su encuesta modificada, y falla con `locked`.