[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Adjunta una encuesta a un comentario existente, o establece el estado completo de la encuesta que ya tiene.

El cuerpo es la encuesta completa, y las opciones que envías se convierten en las opciones de la encuesta, en ese orden. Cada opción
se corresponde por su `id`:

- Una opción enviada con el `id` de una opción existente mantiene esa opción y sus votos. Su etiqueta y posición
  se actualizan a lo que enviaste.
- Una opción enviada sin un `id` se agrega, sin votos.
- Una opción existente que omites se elimina, junto con los votos emitidos en ella. `totalVotes` disminuye en la misma
  cantidad.

Así que para agregar una opción, envía las opciones actuales con sus ids más la nueva sin id. Para eliminar una,
envía la lista sin ella. Los ids de las opciones están en la encuesta devuelta por `GET /api/v1/polls/:commentId`.

Enviar ningún id reemplaza cada opción y elimina todos los votos ya emitidos en la encuesta. Si la encuesta tiene
votos, esto requiere `replaceVotes=true`, y sin ello la API responde con `replace-votes-required`.

Los demás campos también se reemplazan: omitir `closesAt`, `privacy` o `requireVoteToSeeResults` lo restablece a
su valor predeterminado. Para cambiar un solo campo y dejar el resto intacto, usa `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Ejemplo cURL de Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Requerido para no mantener ninguno de los ids de opciones existentes cuando la encuesta tiene votos, ya que eso los elimina a todos. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** El id de una opción existente, para mantenerla y sus votos. Omitir para agregar una nueva opción. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** La lista completa y ordenada. Las opciones existentes omitidas se eliminan junto con sus votos. **/
    options: PollPutOption[]
    /** Debe estar en el futuro cuando el comentario aún no tiene encuesta. Omitir para una encuesta que permanezca abierta. **/
    closesAt?: string | null
    /** 0 anónimo (predeterminado), 1 administradores y moderadores, 2 todos. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Incluido en caso de error. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Un `id` que no está en la encuesta, o el mismo `id` dado dos veces, falla con `poll-invalid`. Un comentario sin
  encuesta aún no tiene ids de opción, por lo que cada opción enviada a él debe omitir `id`.
- La privacidad de la encuesta puede restringirse pero no ampliarse una vez que tiene votos.
- Esta API respeta la configuración de tu sitio. Si las encuestas no están habilitadas para el sitio o la página, falla con
  `polls-disabled`.
- Un comentario bloqueado no puede cambiar su encuesta, y falla con `locked`.
- Los widgets conectados se actualizan en tiempo real, por lo que los espectadores ven la nueva encuesta sin recargar.