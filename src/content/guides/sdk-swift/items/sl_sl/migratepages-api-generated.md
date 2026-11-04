Uvrsti paketno opravilo, ki premakne komentarje, strani in povezane zapise z enega urlId ali domene na drugega.
Migracije "urlId" se ujemajo samo z natančnim urlId. Migracije "domain" poiščejo/zamenjajo vrednost v vsakem urlId in url.
Poizveduj GET /api/v1/batch-jobs/{batchJobId}: ko je opravilo končano, vsebuje rezultate posameznih migracij in undoId.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave jih prijavite prek http://github.com/OpenAPITools/openapi-generator/issues/new
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