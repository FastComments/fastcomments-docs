## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Response

Rückgabe: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIGetCommentTextResponse.swift)

## Example

[inline-code-attrs-start title = 'getCommentText Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetCommentTextOptions(editKey: editKey, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]