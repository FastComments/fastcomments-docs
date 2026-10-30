Custom react counts for a page, optionally with the users who reacted.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "\"public\" laat gebruikers met privéprofielen weg, \"include-private\" neemt ze op." |

## Response

Returns: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getPageReacts Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in de bètafase. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" laat gebruikers met privéprofielen weg, \"include-private\" neemt ze op. (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
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