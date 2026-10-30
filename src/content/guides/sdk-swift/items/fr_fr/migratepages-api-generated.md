---
Met en file d'attente un travail par lots qui déplace les commentaires, les pages et les enregistrements associés d'un urlId ou d'un domaine à un autre.  
"urlId" migrations ne correspondent qu'à l'urlId exact. "domain" migrations recherchent/remplacent la valeur à l'intérieur de chaque urlId et url.  
Interrogez GET /api/v1/batch-jobs/{batchJobId} : une fois terminé, il renvoie les résultats par migration et un undoId.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Retourne : [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
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

---