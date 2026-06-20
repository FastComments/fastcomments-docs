## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ReactFeedPostResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di reactFeedPostPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let reactBodyParams = ReactBodyParams(reactType: "reactType_example") // ReactBodyParams | 
let isUndo = true // Bool |  (opzionale)
let broadcastId = "broadcastId_example" // String |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

PublicAPI.reactFeedPostPublic(tenantId: tenantId, postId: postId, reactBodyParams: reactBodyParams, isUndo: isUndo, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]