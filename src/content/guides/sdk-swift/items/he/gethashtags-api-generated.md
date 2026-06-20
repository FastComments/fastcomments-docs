---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | number | query | לא |  |

## תגובה

מחזיר: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetHashTagsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getHashTags'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// הדוגמאות הבאות לקוד עדיין בבטא. לכל בעיה, נא לדווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
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