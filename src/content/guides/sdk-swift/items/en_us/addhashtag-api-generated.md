## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## Response

Returns: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddHashTag200Response.swift)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. If you encounter any issues, please report them via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |  (optional)
let createHashTagBody = CreateHashTagBody(tenantId: "tenantId_example", tag: "tag_example", url: "url_example") // CreateHashTagBody |  (optional)

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