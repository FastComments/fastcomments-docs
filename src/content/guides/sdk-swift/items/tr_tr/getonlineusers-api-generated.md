Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName'i gönderin. |
| afterUserId | string | query | No | İmleç bağlayıcı: önceki yanıttan nextAfterUserId'i gönderin. afterName ayarlandığında, isim eşleşmeleri nedeniyle girişlerin düşmemesi için gereklidir. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ betadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine raporlayın
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
let afterName = "afterName_example" // String | İmleç: önceki yanıttan nextAfterName'i gönderin. (optional)
let afterUserId = "afterUserId_example" // String | İmleç bağlayıcı: önceki yanıttan nextAfterUserId'i gönderin. afterName ayarlandığında, isim eşleşmeleri nedeniyle girişlerin düşmemesi için gereklidir. (optional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
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