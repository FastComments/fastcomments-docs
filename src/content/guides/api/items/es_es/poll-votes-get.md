[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Enumera los votos individuales detrás de los recuentos de una encuesta, de más antiguo a más reciente. Un crédito por cada 100 votos devueltos.

Una encuesta pertenece a un comentario, por lo que los votos se leen una encuesta a la vez y se requiere `commentId`. Refina más con `voterId` para comprobar cómo votó una persona, o con `optionId` para listar a todos los que eligieron una opción determinada.

Se devuelven como máximo 1000 votos por llamada. Usa `skip` para paginar y obtener más.

Se respeta la configuración `privacy` de la encuesta: los votos de una encuesta anónima no pueden leerse, y la solicitud falla con `poll-anonymous`. Consulta la estructura `PollVote` para más detalles.

[inline-code-attrs-start title = 'Ejemplo cURL de PollVotes Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estructura de respuesta PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Incluido en caso de error. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Contar votos por opción

No tienes que sumar estos valores para obtener los resultados; la encuesta lleva sus propios recuentos. Lee la encuesta con `GET /api/v1/polls/:commentId` en su lugar, y usa esta API cuando necesites saber quién votó.

### Cada encuesta en una página

No existe una lista de votos a nivel de página. Para informar de una página completa, obtén sus comentarios con `GET /api/v1/comments`, lo que devuelve la encuesta de cada comentario y sus recuentos, y luego lee los votos de las encuestas que te interesan.