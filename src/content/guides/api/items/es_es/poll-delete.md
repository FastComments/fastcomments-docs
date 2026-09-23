[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Elimina una encuesta de su comentario, junto con cada voto emitido en ella. El propio comentario queda sin cambios.

Eliminar el comentario también elimina su encuesta y los votos, por lo que esto solo es necesario cuando deseas conservar el comentario.

[inline-code-attrs-start title = 'Ejemplo cURL de eliminación de encuesta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de eliminación de encuesta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de eliminación de encuesta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]