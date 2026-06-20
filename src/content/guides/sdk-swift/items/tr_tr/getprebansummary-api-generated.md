## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| includeByUserIdAndEmail | boolean | query | Hayır |  |
| includeByIP | boolean | query | Hayır |  |
| includeByEmailDomain | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## Örnek

[inline-code-attrs-start title = 'getPreBanSummary Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (isteğe bağlı)
let includeByIP = true // Bool |  (isteğe bağlı)
let includeByEmailDomain = true // Bool |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.getPreBanSummary(commentId: commentId, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]