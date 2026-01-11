## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserReactsPublic200Response.swift)

## Example

[inline-code-attrs-start title = 'getUserReactsPublic Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getUserReactsPublic(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
