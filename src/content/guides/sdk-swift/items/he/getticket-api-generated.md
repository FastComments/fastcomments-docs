## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |

## תגובה

מחזיר: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTicket200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTicket'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בבטא. עבור כל בעיה, נא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let userId = "userId_example" // String |  (אופציונלי)

DefaultAPI.getTicket(tenantId: tenantId, id: id, userId: userId) { (response, error) in
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