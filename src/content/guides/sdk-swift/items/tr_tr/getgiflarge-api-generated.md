---
## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| largeInternalURLSanitized | string | query | Evet |  |

## Yanıt

Döndürür: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GifGetLargeResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getGifLarge Örnek'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta durumundadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let largeInternalURLSanitized = "largeInternalURLSanitized_example" // String | 

PublicAPI.getGifLarge(tenantId: tenantId, largeInternalURLSanitized: largeInternalURLSanitized) { (response, error) in
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