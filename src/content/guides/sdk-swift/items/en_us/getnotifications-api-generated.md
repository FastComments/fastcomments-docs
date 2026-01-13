## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |
| skip | number | query | No |  |

## Response

Returns: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotifications200Response.swift)

## Example

[inline-code-attrs-start title = 'getNotifications Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. If you encounter any issues, please report them at http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optional)
let urlId = "urlId_example" // String |  (optional)
let fromCommentId = "fromCommentId_example" // String |  (optional)
let viewed = true // Bool |  (optional)
let type = "type_example" // String |  (optional)
let skip = 987 // Double |  (optional)

DefaultAPI.getNotifications(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip) { (response, error) in
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