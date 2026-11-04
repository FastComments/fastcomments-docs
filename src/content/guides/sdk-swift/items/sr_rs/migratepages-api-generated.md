Стави у ред серијски посао који премешта коментаре, странице и повезане записе са једног urlId или домена на други.
„urlId" миграције поклапају само тачан urlId. „domain" миграције траже/замењују вредност унутар сваког urlId и url.
Питај GET /api/v1/batch-jobs/{batchJobId}: када је завршен, садржи резултате по миграцији и undoId.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'migratePages Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још увек у бета верзији. За било какав проблем, пријавите преко http://github.com/OpenAPITools/openapi-generator/issues/new
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