Met en file d'attente un travail batch qui annule chaque migration d'un travail migrate-pages, du plus récent au plus ancien. Disponible pendant 90 jours.  
Les enregistrements modifiés depuis la migration sont laissés intacts, de sorte qu'une modification ultérieure n'est jamais écrasée.  
Interrogez GET /api/v1/batch-jobs/{batchJobId} pour obtenir les comptes restaurés/ignorés.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| undoId | string | path | Oui |  |

## Réponse

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
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