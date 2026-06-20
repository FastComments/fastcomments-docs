## Parametreler

| Ad | Tür | Konum | Gerekiyor mu | Açıklama |
|------|------|----------|----------|-------------|
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersCountResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getCounts Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.getCounts(sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]