Like count for a page, optionally with the users who liked it.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "\"public\" omet les utilisateurs avec des profils privés, \"include-private\" les inclut." |

## Response

Renvoie: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getPageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // Chaîne | 
let urlId = "urlId_example" // Chaîne | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" omet les utilisateurs avec des profils privés, \"include-private\" les inclut. (optionnel)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]