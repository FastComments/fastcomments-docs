
req
tenantId
afterId

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## Response

Returns: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## Example

[inline-code-attrs-start title = 'getFeedPostsPublic Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (optional)
let limit = 987 // Int |  (optional)
let tags = ["inner_example"] // [String] |  (optional)
let sso = "sso_example" // String |  (optional)
let isCrawler = true // Bool |  (optional)
let includeUserInfo = true // Bool |  (optional)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
