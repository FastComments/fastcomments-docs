## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí | El inquilino del que leer los registros de auditoría. |
| limit | integer | query | No | Cuántos registros devolver. Máximo 10000, predeterminado a 1000. |
| skip | integer | query | No | Cuántos registros omitir, para paginación. |
| order | string | query | No | Dirección de ordenación en la hora del evento. Predeterminado a DESC (más reciente primero). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Solo devolver eventos realizados por este nombre de usuario. |
| ip | string | query | No | Solo devolver eventos de esta dirección IP. |
| crudType | string | query | No | Solo devolver eventos de este tipo: c, r, u, d, o login. |
| resourceName | string | query | No | Solo devolver eventos para este recurso, p. ej. User o Moderator. |
| targetId | string | query | No | Solo devolver eventos cuyo objeto afectado tenga este id. |
| target | string | query | No | Coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta del objeto afectado, p. ej. un nombre de usuario o correo electrónico. |
| includeManagedTenants | boolean | query | No | También devolver eventos de los inquilinos que este inquilino gestiona. |

## Respuesta

Devuelve: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'getAuditLogs Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, por favor informe a http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | El inquilino del que leer los registros de auditoría.
let limit = 987 // Int | Cuántos registros devolver. Máximo 10000, predeterminado a 1000. (opcional)
let skip = 987 // Int | Cuántos registros omitir, para paginación. (opcional)
let order = SORT_DIR() // SORTDIR | Dirección de ordenación en la hora del evento. Predeterminado a DESC (más reciente primero). (opcional)
let after = 987 // Int64 |  (opcional)
let before = 987 // Int64 |  (opcional)
let username = "username_example" // String | Solo devolver eventos realizados por este nombre de usuario. (opcional)
let ip = "ip_example" // String | Solo devolver eventos de esta dirección IP. (opcional)
let crudType = "crudType_example" // String | Solo devolver eventos de este tipo: c, r, u, d, o login. (opcional)
let resourceName = "resourceName_example" // String | Solo devolver eventos para este recurso, p. ej. User o Moderator. (opcional)
let targetId = "targetId_example" // String | Solo devolver eventos cuyo objeto afectado tenga este id. (opcional)
let target = "target_example" // String | Coincidencia de subcadena sin distinción de mayúsculas/minúsculas en la etiqueta del objeto afectado, p. ej. un nombre de usuario o correo electrónico. (opcional)
let includeManagedTenants = true // Bool | También devolver eventos de los inquilinos que este inquilino gestiona. (opcional)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]