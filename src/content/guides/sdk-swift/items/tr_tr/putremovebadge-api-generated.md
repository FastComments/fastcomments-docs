## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| badgeId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| commentId | string | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RemoveUserBadgeResponse.swift)

## Örnek

[inline-code-attrs-start title = 'putRemoveBadge Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hala beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (isteğe bağlı)
let commentId = "commentId_example" // String |  (isteğe bağlı)
let broadcastId = "broadcastId_example" // String |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

ModerationAPI.putRemoveBadge(badgeId: badgeId, userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
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