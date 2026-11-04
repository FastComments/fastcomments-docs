מתור עבודת אצווה (batch job) שמעבירה תגובות, דפים ורשומות קשורות מ-urlId או דומיין אחד לאחר.
העברות מסוג "urlId" מתאימות רק ל-urlId המדויק. העברות מסוג "domain" מבצעות חיפוש/החלפה של הערך בתוך כל urlId ו-url.
יש לבצע Polling ל-GET /api/v1/batch-jobs/{batchJobId}: ברגע שהעבודה מסתיימת, היא כוללת תוצאות לכל העברה ו-undoId.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תשובה

מחזיר: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-migratePages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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