req
tenantId
urlId
userIdWS

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| userIdWS | string | query | Evet |  |
| startTime | integer | query | Evet |  |
| endTime | integer | query | Hayır |  |

## Yanıt

Döndürür: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEventLogResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getGlobalEventLog Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userIdWS = "userIdWS_example" // String | 
let startTime = 987 // Int64 | 
let endTime = 987 // Int64 |  (isteğe bağlı)

PublicAPI.getGlobalEventLog(tenantId: tenantId, urlId: urlId, userIdWS: userIdWS, startTime: startTime, endTime: endTime) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]