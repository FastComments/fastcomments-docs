Bir sayfanın şu anda çevrimiçi izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler, saymadığımız anonim izleyiciler dahil).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini gönderin. |
| afterUserId | string | query | No | İmleç eşitlik bozucusu: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim eşitliklerinin girişlerin atlanmasına neden olmaması için gereklidir. |

## Response

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
let afterName = "afterName_example" // String | İmleç: önceki yanıttan nextAfterName değerini gönderin. (isteğe bağlı)
let afterUserId = "afterUserId_example" // String | İmleç eşitlik bozucusu: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim eşitliklerinin girişlerin atlanmasına neden olmaması için gereklidir. (isteğe bağlı)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]