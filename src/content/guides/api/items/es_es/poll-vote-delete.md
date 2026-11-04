[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Retira un voto. La opción en la que fue emitido recupera su conteo, y el votante queda libre para votar de nuevo.

[inline-code-attrs-start title = 'Ejemplo de cURL para eliminar PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud de eliminación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta de eliminación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de fallo. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Incluido en caso de fallo. **/
    reason?: string
    /** La encuesta con sus conteos actualizados. **/
    poll: CommentPoll
}
[inline-code-end]

### Otras notas

- Eliminar el mismo voto dos veces responde con `not-found` la segunda vez, y los conteos se dejan sin cambios.
- Si la encuesta fue reemplazada desde que se emitió el voto, el voto se elimina pero no hay cambios en los conteos, ya que
  el reemplazo comenzó desde cero.