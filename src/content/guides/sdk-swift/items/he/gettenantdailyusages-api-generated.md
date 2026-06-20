---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| yearNumber | number | שאילתה | לא |  |
| monthNumber | number | שאילתה | לא |  |
| dayNumber | number | שאילתה | לא |  |
| skip | number | שאילתה | לא |  |

## תגובה

מחזיר: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsagesResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של getTenantDailyUsages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (אופציונלי)
let monthNumber = 987 // Double |  (אופציונלי)
let dayNumber = 987 // Double |  (אופציונלי)
let skip = 987 // Double |  (אופציונלי)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
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