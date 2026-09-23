[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Revierte un voto. La opción en la que se emitió devuelve su recuento, y el votante puede volver a votar.

[inline-code-attrs-start title = 'Ejemplo cURL de eliminación de PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de eliminación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de eliminación de PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Incluido en caso de error. **/
    reason?: string
    /** La encuesta con sus recuentos actualizados. **/
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Eliminar el mismo voto dos veces responde con `not-found` la segunda vez, y los recuentos permanecen sin cambios.
- Si la encuesta fue reemplazada desde que se emitió el voto, el voto se elimina pero no hay cambios en el recuento, ya que el reemplazo comenzó desde cero.

---