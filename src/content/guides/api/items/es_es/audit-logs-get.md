[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Esta API usa paginación, proporcionada por los parámetros `skip`, `before` y `after`. Los AuditLogs se devuelven en páginas de `1000`, ordenados por `when` e `id`.

Obtener cada `1000` registros tiene un costo de crédito de `10`.

Por defecto, recibirá una lista con **los elementos más nuevos primero**. De esta manera, puede consultar comenzando con `skip=0`, paginando hasta encontrar el último registro que ha consumido.

Alternativamente, puede ordenar del más antiguo al más nuevo, y paginar hasta que no haya más registros.

La ordenación se puede hacer configurando `order` a `ASC` o `DESC`. El valor predeterminado es `ASC`.

La consulta por fecha es posible a través de `before` y `after` como marcas de tiempo con milisegundos. `before` y `after` NO son inclusivos.

[inline-code-attrs-start title = 'Ejemplo cURL de AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
