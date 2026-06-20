## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| sure | string | query | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בבטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let sure = "sure_example" // String |  (אופציונלי)

DefaultAPI.deleteTenant(tenantId: tenantId, id: id, sure: sure) { (response, error) in
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