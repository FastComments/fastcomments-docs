## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Response

Returns: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteCommentResult.swift)

## Example

[inline-code-attrs-start title = 'Exemple deleteComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les échantillons de code suivants sont encore bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (facultatif)
let isLive = true // Bool |  (facultatif)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, options: DefaultAPI.DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]