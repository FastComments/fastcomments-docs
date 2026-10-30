מכניס לתור משימת אצווה שמבטלת כל הגירה ממשימת migrate-pages אחת, מהחדשה ביותר. זמינה למשך 90 יום.  
רשומות שהשתנו מאז ההגירה נותרות ללא שינוי, כך ששינוי מאוחר יותר לעולם לא יידרס.  
בצע בקשת GET ל‑/api/v1/batch-jobs/{batchJobId} כדי לקבל את ספירות השחזורים/הדילוגים.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| undoId | string | path | Yes |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'דוגמה ל-undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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