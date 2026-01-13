## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## Примјер

[inline-code-attrs-start title = 'getPendingWebhookEvents Примјер'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су и даље бета. За било који проблем, пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (опционо)
let externalId = "externalId_example" // String |  (опционо)
let eventType = "eventType_example" // String |  (опционо)
let type = "type_example" // String |  (опционо)
let domain = "domain_example" // String |  (опционо)
let attemptCountGT = 987 // Double |  (опционо)
let skip = 987 // Double |  (опционо)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]