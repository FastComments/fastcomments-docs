## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| postId | string | path | Sim |  |
| isUndo | boolean | query | Não |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Response

Retorna: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ReactFeedPostResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'reactFeedPostPublic Exemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let reactBodyParams = ReactBodyParams(reactType: "reactType_example") // ReactBodyParams | 
let isUndo = true // Bool |  (opcional)
let broadcastId = "broadcastId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

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

---