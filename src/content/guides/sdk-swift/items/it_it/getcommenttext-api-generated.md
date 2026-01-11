## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentText200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, editKey: editKey, sso: sso) { (response, error) in
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