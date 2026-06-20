## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| postId | string | path | Tak |  |
| isUndo | boolean | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ReactFeedPostResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład reactFeedPostPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w fazie beta. W przypadku problemu zgłoś go pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let reactBodyParams = ReactBodyParams(reactType: "reactType_example") // ReactBodyParams | 
let isUndo = true // Bool |  (opcjonalne)
let broadcastId = "broadcastId_example" // String |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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