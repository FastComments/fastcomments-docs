[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Esta API utiliza paginación, provista por los parámetros `skip`, `limit`, `before` y `after`. Los AuditLogs se devuelven en páginas de `5000` por defecto, hasta un `limit` máximo de `10000`, ordenados por `when` e `id`. Las páginas son grandes porque este endpoint suele usarse para volcar el historial en lugar de paginarlo interactivamente.

Cada `100` registros devueltos tiene un costo de crédito de `1`.

Por defecto, recibirás una lista con **los elementos más recientes primero**. De esta manera, puedes hacer polling comenzando con `skip=0`, paginando hasta que encuentres el último registro que has consumido.

Alternativamente, puedes ordenar de más antiguo a más reciente, y paginar hasta que no haya más registros.

El ordenamiento se puede hacer estableciendo `order` a `ASC` o `DESC`. El valor por defecto es `DESC`.

La consulta por fecha es posible mediante `before` y `after` como marcas de tiempo en milisegundos. `before` y `after` NO son inclusivos, y cualquiera de ellos puede usarse por separado.

## Finding what happened to a person

## Encontrar lo que le sucedió a una persona

Cada evento registra quién lo realizó (`username`, `userId`, `ip`) y, por separado, sobre qué se realizó. `targetLabel` es una etiqueta legible para ese objeto, por ejemplo `jsmith (jsmith@example.com)`, y `targetId` es su id. Usa `target` para una coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta cuando conoces el nombre o correo electrónico de una persona pero no su id.

Los borrados capturan la etiqueta en el momento del evento, de modo que un usuario o moderador eliminado aún puede ser identificado después de que el registro subyacente haya desaparecido.

## Managed tenants

## Tenants gestionados

Si tu tenant gestiona otros tenants, establece `includeManagedTenants=true` para devolver eventos de tu tenant y de cada tenant que gestiona en una sola respuesta. El `tenantId` de cada registro devuelto indica de qué tenant proviene.

[inline-code-attrs-start title = 'Ejemplo cURL de AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Máximo 10000. Por defecto 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Solo eventos realizados por este nombre de usuario. **/
    username?: string
    /** Solo eventos de esta dirección IP. **/
    ip?: string
    /** Solo eventos de este tipo. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Solo eventos para este recurso, p. ej. Usuario o Moderador. **/
    resourceName?: string
    /** Solo eventos cuyo objeto afectado tiene este id. **/
    targetId?: string
    /** Coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta del objeto afectado. **/
    target?: string
    /** También devuelve eventos de los tenants que este tenant gestiona. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Incluido en caso de error. **/
    reason?: string
    /** ¡Los registros! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---