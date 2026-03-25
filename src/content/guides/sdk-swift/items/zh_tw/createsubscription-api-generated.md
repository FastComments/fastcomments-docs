## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳：[`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateSubscriptionAPIResponse.swift)

## 範例

[inline-code-attrs-start title = 'createSubscription 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍為 Beta 版本。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(notificationFrequency: 123, pageTitle: "pageTitle_example", url: "url_example", urlId: "urlId_example", anonUserId: "anonUserId_example", userId: "userId_example") // CreateAPIUserSubscriptionData | 

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