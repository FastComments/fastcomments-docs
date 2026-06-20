Sayfada daha önce yorum yapmış, şu anda çevrim içi olmayan kişiler. displayName'e göre sıralanır.
/users/online tükendikten sonra bir "Üyeler" bölümünü görüntülemek için bunu kullanın.
commenterName üzerinde imleçli sayfalandırma: sunucu kısmi {tenantId, urlId, commenterName}
indeksini afterName'den itibaren $gt ile ileri doğru yürütür, $skip maliyeti yok.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Page URL identifier (cleaned server-side). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini geçin. |
| afterUserId | string | query | Hayır | İmleç eşitleyici: önceki yanıttan nextAfterUserId değerini geçin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Page URL identifier (cleaned server-side).
let afterName = "afterName_example" // String | İmleç: önceki yanıttan nextAfterName değerini geçin. (isteğe bağlı)
let afterUserId = "afterUserId_example" // String | İmleç eşitleyici: önceki yanıttan nextAfterUserId değerini geçin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir. (isteğe bağlı)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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