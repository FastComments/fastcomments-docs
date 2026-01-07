## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Response

Returns: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteComment200Response.swift)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let contextUserId = "contextUserId_example" // String |  (optional)
let isLive = true // Bool |  (optional)

DefaultAPI.deleteComment(tenantId: tenantId, id: id, contextUserId: contextUserId, isLive: isLive) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
