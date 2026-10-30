## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| title | string | query | לא |  |
| sso | string | query | לא |  |

## Response

Returns: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Example

[inline-code-attrs-start title = 'דוגמת createV1PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בגרסת בטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // מחרוזת | 
let urlId = "urlId_example" // מחרוזת | 
let title = "title_example" // מחרוזת |  (אופציונלי)
let sso = "sso_example" // מחרוזת |  (אופציונלי)

PublicAPI.createV1PageReact(tenantId: tenantId, urlId: urlId, options: PublicAPI.CreateV1PageReactOptions(title: title, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]