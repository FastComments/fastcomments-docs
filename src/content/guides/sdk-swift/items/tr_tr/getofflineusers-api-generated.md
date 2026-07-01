---
Sayfada daha önce yorum yapmış ancak şu anda çevrimiçi olmayan yorumcular. displayName göre sıralanır.  
Bu, /users/online kapsamını tükettikten sonra bir “Members” (Üyeler) bölümü oluşturmak için kullanılır.  
commenterName üzerinde imleç sayfalama: sunucu, {tenantId, urlId, commenterName} kısmını afterName'den itibaren $gt ile ilerleyerek yürütür, $skip maliyeti yok.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenmiş). |
| afterName | string | query | No | İmleç: bir önceki yanıttan nextAfterName'i geç. |
| afterUserId | string | query | No | İmleç bağlayıcı: bir önceki yanıttan nextAfterUserId'i geç. afterName ayarlandığında gerekli, böylece isim eşleşmeleri girdileri düşürmez. |

## Response

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenmiş).
let afterName = "afterName_example" // String | İmleç: bir önceki yanıttan nextAfterName'i geç. (isteğe bağlı)
let afterUserId = "afterUserId_example" // String | İmleç bağlayıcı: bir önceki yanıttan nextAfterUserId'i geç. afterName ayarlandığında gerekli, böylece isim eşleşmeleri girdileri düşürmez. (isteğe bağlı)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]