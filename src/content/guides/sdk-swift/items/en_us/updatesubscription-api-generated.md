## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Response

Returns: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateSubscriptionAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'updateSubscription Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(notificationFrequency: 123) // UpdateAPIUserSubscriptionData | 
let userId = "userId_example" // String |  (optional)

DefaultAPI.updateSubscription(tenantId: tenantId, id: id, updateAPIUserSubscriptionData: updateAPIUserSubscriptionData, userId: userId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
