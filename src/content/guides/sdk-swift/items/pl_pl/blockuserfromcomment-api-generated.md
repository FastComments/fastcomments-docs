## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład blockUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let blockFromCommentParams = BlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // BlockFromCommentParams | 
let userId = "userId_example" // String |  (opcjonalne)
let anonUserId = "anonUserId_example" // String |  (opcjonalne)

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