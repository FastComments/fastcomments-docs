## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| sso | string | query | Hayır |  |
| includeUserInfo | string | query | Hayır | Ayarlandığında, sayfayı beğenen kullanıcıları (avatar ve kullanıcı adlarını) döndürür. Yalnızca "public" kabul edilir, bu da özel profilleri dışarı bırakır. |

## Yanıt

Döndürür: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Örnek

[inline-code-attrs-start title = 'getV1PageLikes Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (isteğe bağlı)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Ayarlandığında, sayfayı beğenen kullanıcıları (avatar ve kullanıcı adlarını) döndürür. Yalnızca \"public\" kabul edilir, bu da özel profilleri dışarı bırakır. (isteğe bağlı)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]