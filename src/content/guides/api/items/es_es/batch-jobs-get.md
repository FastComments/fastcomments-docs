[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Esta ruta devuelve los trabajos por lotes del inquilino de todos los tipos, más recientes primero, 100 por página. Use `type` y `status` para limitar la lista, y `skip` para paginarla.

[inline-code-attrs-start title = 'Ejemplo cURL de Lista de BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Lista de BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Sólo trabajos de este tipo, por ejemplo migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Cuántos trabajos omitir, para paginación. Un entero no negativo. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Lista de BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Incluido en caso de error. **/
    reason?: string
    /** Más recientes primero, como máximo 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]