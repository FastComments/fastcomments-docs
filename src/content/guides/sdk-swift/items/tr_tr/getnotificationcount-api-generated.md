## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| urlId | string | query | Hayır |  |
| fromCommentId | string | query | Hayır |  |
| viewed | boolean | query | Hayır |  |
| type | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getNotificationCount Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (isteğe bağlı)
let urlId = "urlId_example" // String |  (isteğe bağlı)
let fromCommentId = "fromCommentId_example" // String |  (isteğe bağlı)
let viewed = true // Bool |  (isteğe bağlı)
let type = "type_example" // String |  (isteğe bağlı)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]