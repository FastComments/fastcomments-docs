Στην ουρά μια εργασία δέσμης που μεταφέρει σχόλια, σελίδες και σχετικά αρχεία από ένα urlId ή domain σε άλλο.
Οι μετακινήσεις "urlId" ταιριάζουν μόνο το ακριβές urlId. Οι μετακινήσεις "domain" κάνουν αναζήτηση/αντικατάσταση της τιμής μέσα σε κάθε urlId και url.
Κάντε poll στο GET /api/v1/batch-jobs/{batchJobId}: όταν ολοκληρωθεί, περιέχει αποτελέσματα ανά μετακίνηση και ένα undoId.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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