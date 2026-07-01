Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página neste momento.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Retorna anonCount + totalCount (assinantes de toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | No | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | No | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não removam entradas. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador da URL da página (limpo no servidor).
let afterName = "afterName_example" // String | Cursor: passe nextAfterName da resposta anterior. (optional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não removam entradas. (optional)

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