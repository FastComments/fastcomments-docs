---
Enfileira um trabalho em lote que reverte todas as migrações de um trabalho migrate-pages, da mais recente para a mais antiga. Disponível por 90 dias.  
Registros alterados desde a migração são deixados como estão, de modo que uma edição posterior nunca seja sobrescrita.  
Faça polling GET /api/v1/batch-jobs/{batchJobId} para obter as contagens de restaurados/ignorados.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| undoId | string | path | Yes |  |

## Response

Retorna: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations Exemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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