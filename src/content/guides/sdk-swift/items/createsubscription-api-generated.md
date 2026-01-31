## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateSubscriptionAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'createSubscription Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(pageTitle: "pageTitle_example", url: "url_example", urlId: "urlId_example", anonUserId: "anonUserId_example", userId: "userId_example") // CreateAPIUserSubscriptionData | 

DefaultAPI.createSubscription(tenantId: tenantId, createAPIUserSubscriptionData: createAPIUserSubscriptionData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
