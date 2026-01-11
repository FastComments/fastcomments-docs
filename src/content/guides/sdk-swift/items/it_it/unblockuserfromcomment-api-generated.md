## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Risposta

Restituisce: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UnBlockCommentPublic200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di unBlockUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let unBlockFromCommentParams = UnBlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // UnBlockFromCommentParams | 
let userId = "userId_example" // String |  (opzionale)
let anonUserId = "anonUserId_example" // String |  (opzionale)

DefaultAPI.unBlockUserFromComment(tenantId: tenantId, id: id, unBlockFromCommentParams: unBlockFromCommentParams, userId: userId, anonUserId: anonUserId) { (response, error) in
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