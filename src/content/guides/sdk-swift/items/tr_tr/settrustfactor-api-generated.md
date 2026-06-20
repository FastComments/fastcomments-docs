## Parametreler

| Ad | Tür | Konum | Gereklidir | Açıklama |
|------|------|----------|----------|-------------|
| userId | string | query | Hayır |  |
| trustFactor | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Örnek

[inline-code-attrs-start title = 'setTrustFactor Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresi üzerinden bildirin
import FastCommentsSwift

let userId = "userId_example" // String |  (isteğe bağlı)
let trustFactor = "trustFactor_example" // String |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]