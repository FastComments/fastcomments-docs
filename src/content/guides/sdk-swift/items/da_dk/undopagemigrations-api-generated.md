Sætter et batch-job i kø, der tilbagefører hver migration fra ét migrate-pages-job, nyeste først. Tilgængeligt i 90 dage.
Poster, der er ændret siden migrationen, efterlades urørte, så en senere redigering aldrig overskrives.
Poll GET /api/v1/batch-jobs/{batchJobId} for at se antallet af gendannede/sprungne over.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| undoId | string | path | Ja |  |

## Svar

Returnerer: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'undoPageMigrations Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Rapportér venligst eventuelle problemer via http://github.com/OpenAPITools/openapi-generator/issues/new
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