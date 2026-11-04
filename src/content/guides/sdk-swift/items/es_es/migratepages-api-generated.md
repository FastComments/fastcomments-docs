Pone en cola un trabajo por lotes que mueve comentarios, páginas y registros relacionados de un urlId o dominio a otro.
Las migraciones por "urlId" solo coinciden con el urlId exacto. Las migraciones por "domain" buscan/reemplazan el valor dentro de cada urlId y url.
Consulte con GET /api/v1/batch-jobs/{batchJobId}: una vez finalizado, contiene los resultados de cada migración y un undoId.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let migrateAPIPagesData = MigrateAPIPagesData(migrations: [PageMigration(from: "from_example", to: "to_example", type: PageMigrationType())]) // MigrateAPIPagesData | 

DefaultAPI.migratePages(tenantId: tenantId, migrateAPIPagesData: migrateAPIPagesData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]