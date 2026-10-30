## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Ayarlandığında, her reaksiyon kimliği için kullanıcıları (avatarlar ve kullanıcı adları) döndürür. Yalnızca \"public\" kabul edilir, bu da özel profilleri dışarı bırakır. |

## Yanıt

Döndürür: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Örnek

[inline-code-attrs-start title = 'getV2PageReacts Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String | (opsiyonel)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Ayarlandığında, her reaksiyon kimliği için kullanıcıları (avatarlar ve kullanıcı adları) döndürür. Yalnızca \"public\" kabul edilir, bu da özel profilleri dışarı bırakır. (opsiyonel)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]