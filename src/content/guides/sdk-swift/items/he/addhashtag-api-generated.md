## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | לא |  |

## תגובה

מחזיר: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTag200Response.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-addHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (אופציונלי)
let createHashTagBody = CreateHashTagBody(tenantId: "tenantId_example", tag: "tag_example", url: "url_example") // CreateHashTagBody |  (אופציונלי)

DefaultAPI.addHashTag(tenantId: tenantId, createHashTagBody: createHashTagBody) { (response, error) in
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