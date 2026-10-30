---
Encola un trabajo por lotes que revierte cada migración de un trabajo migrate‑pages, de más reciente a más antiguo. Disponible durante 90 días.  
Los registros modificados desde la migración se dejan sin cambios, de modo que una edición posterior nunca se sobrescribe.  
Realice una solicitud GET a `/api/v1/batch-jobs/{batchJobId}` para obtener los recuentos de restaurados/omitidos.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| undoId | string | path | Sí |  |

## Respuesta

Devuelve: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'undoPageMigrations Ejemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código siguen en beta. Para cualquier problema, por favor repórtelo a http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---