Bir urlId veya domain'den diğerine yorumları, sayfaları ve ilgili kayıtları taşıyan bir toplu iş (batch job) kuyruğa alır.
"urlId" geçişleri yalnızca tam olarak eşleşen urlId ile çalışır. "domain" geçişleri her urlId ve url içindeki değeri bul/değiştir yapar.
GET /api/v1/batch-jobs/{batchJobId} uç noktasını sorgulayın: işlem tamamlandığında geçiş başına sonuçları ve bir undoId içerir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Örnek

[inline-code-attrs-start title = 'migratePages Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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