## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | number | query | לא |  |

## תגובה

מחזיר: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetHashTags200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getHashTags'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Double |  (אופציונלי)

DefaultAPI.getHashTags(tenantId: tenantId, page: page) { (response, error) in
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