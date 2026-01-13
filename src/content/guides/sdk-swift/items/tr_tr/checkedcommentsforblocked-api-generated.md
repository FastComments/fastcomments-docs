---
## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentIds | string | query | Evet | Virgülle ayrılmış yorum kimlikleri listesi. |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckedCommentsForBlocked200Response.swift)

## Örnek

[inline-code-attrs-start title = 'checkedCommentsForBlocked Örnek'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | Virgülle ayrılmış yorum kimlikleri listesi.
let sso = "sso_example" // String |  (isteğe bağlı)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---