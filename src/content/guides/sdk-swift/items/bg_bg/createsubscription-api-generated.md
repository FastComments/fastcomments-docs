## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | заявка | Да |  |

## Отговор

Връща: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateSubscriptionAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за createSubscription'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са в бета. За проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
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