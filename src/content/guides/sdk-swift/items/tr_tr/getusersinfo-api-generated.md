Toplu kullanıcı bilgisi bir kiracı için. Verilen userIds ile User / SSOUser'dan görüntü bilgilerini döndürür.
Yorum widget'ı tarafından, presence olayıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik tutarlı şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tip | Location | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Virgülle ayrılmış userIds. |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | Virgülle ayrılmış userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]