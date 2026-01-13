## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionConfigs200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getQuestionConfigs Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (isteğe bağlı)

DefaultAPI.getQuestionConfigs(tenantId: tenantId, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]