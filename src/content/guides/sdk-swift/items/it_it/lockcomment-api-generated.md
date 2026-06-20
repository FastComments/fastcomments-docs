## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| broadcastId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di lockComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Gli esempi di codice seguenti sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo su http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let broadcastId = "broadcastId_example" // String | 
let sso = "sso_example" // String |  (opzionale)

PublicAPI.lockComment(tenantId: tenantId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]