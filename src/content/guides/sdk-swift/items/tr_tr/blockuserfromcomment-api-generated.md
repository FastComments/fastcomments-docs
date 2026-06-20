## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| id | string | yol | Evet |  |
| userId | string | sorgu | Hayır |  |
| anonUserId | string | sorgu | Hayır |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Örnek

[inline-code-attrs-start title = 'blockUserFromComment Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let blockFromCommentParams = BlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // BlockFromCommentParams | 
let userId = "userId_example" // String |  (isteğe bağlı)
let anonUserId = "anonUserId_example" // String |  (isteğe bağlı)

DefaultAPI.blockUserFromComment(tenantId: tenantId, id: id, blockFromCommentParams: blockFromCommentParams, userId: userId, anonUserId: anonUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]