[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Registra un voto en una encuesta.

Un votante tiene como máximo un voto por encuesta. Llamar a este endpoint nuevamente para el mismo votante mueve su voto a la nueva opción en lugar de agregar uno segundo, y votar por la opción que ya eligió no hace nada.

La respuesta incluye la encuesta, por lo que obtienes los recuentos actualizados sin una segunda solicitud.

[inline-code-attrs-start title = 'Ejemplo cURL de creación de PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Ejemplo cURL de creación de PollVote anónimo'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estructura de solicitud de creación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** Se requiere uno de userId o anonUserId. **/
    userId?: string
    anonUserId?: string
    /** La IP del usuario final, usada para el límite de velocidad anónimo. Por defecto es la IP del llamador. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de creación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Incluido en caso de error. **/
    reason?: string
    pollVote: PollVote
    /** La encuesta con sus recuentos actualizados. **/
    poll: CommentPoll
}
[inline-code-end]

### Votos anónimos

Establezca `anonUserId` en lugar de `userId` para registrar un voto de alguien que no ha iniciado sesión. Ese id no tiene que corresponder a un usuario en ninguna parte; solo identifica la sesión, de modo que la misma persona no se cuente dos veces.

El voto anónimo debe estar habilitado para su sitio. Si la votación está limitada a usuarios registrados, un voto con solo un `anonUserId` falla con `poll-login-required`.

Los votos anónimos también están limitados por tasa por IP por encuesta, para evitar que una persona llene una encuesta borrando su sesión. Envíe la `ip` del usuario final para que el límite se aplique a él en lugar de a su servidor.

### Otras notas

- Un `userId` debe ser un usuario que exista en su sitio. Los votos para un usuario que pertenece a otro sitio son rechazados.
- Votar en una encuesta cerrada falla con `poll-closed`.
- Esta API actualiza los recuentos en la encuesta y los envía a los widgets conectados en tiempo real.