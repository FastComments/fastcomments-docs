[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Esta API utiliza paginación, proporcionada por los parámetros `skip`, `limit`, `before` y `after`. Los AuditLogs se devuelven en páginas de `100` por defecto, hasta un `limit` máximo de `200`, ordenados por `when` e `id`.

Cada `100` registros devueltos tiene un costo de crédito de `1`.

Por defecto, recibirás una lista con **los elementos más nuevos primero**. De esta manera, puedes hacer polling comenzando con `skip=0`, paginando hasta que encuentres el último registro que has consumido.

Alternativamente, puedes ordenar de más antiguo a más reciente, y paginar hasta que no haya más registros.

El ordenamiento se puede hacer estableciendo `order` a `ASC` o `DESC`. El valor predeterminado es `DESC`.

Consultar por fecha es posible mediante `before` y `after` como marcas de tiempo con milisegundos. `before` y `after` NO son inclusivos, y cualquiera de los dos puede usarse por separado.

## Encontrar lo que le sucedió a una persona

Cada evento registra quién lo realizó (`username`, `userId`, `ip`) y, por separado, sobre qué se realizó. `targetLabel` es una etiqueta legible para ese objeto, por ejemplo `jsmith (jsmith@example.com)`, y `targetId` es su id. Usa `target` para una coincidencia de subcadena sin distinción de mayúsculas cuando conozcas el nombre o correo de una persona pero no su id.

Los borrados capturan la etiqueta en el momento del evento, por lo que un usuario o moderador eliminado aún puede ser identificado después de que el registro subyacente haya desaparecido.

## Inquilinos gestionados

Si tu inquilino gestiona otros inquilinos, establece `includeManagedTenants=true` para devolver eventos de tu inquilino y de cada inquilino que gestiona en una sola respuesta. El `tenantId` de cada registro devuelto le indica de qué inquilino proviene.

[inline-code-attrs-start title = 'Ejemplo cURL de AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]