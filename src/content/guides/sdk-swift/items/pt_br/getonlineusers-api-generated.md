Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página neste momento.
Retorna anonCount + totalCount (assinantes em toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificador da URL da página (limpo no lado do servidor). |
| afterName | string | query | No | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | No | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver configurado para que empates de nome não descartem entradas. |

## Response

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo de getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador da URL da página (limpo no lado do servidor).
let afterName = "afterName_example" // String | Cursor: passe nextAfterName da resposta anterior. (opcional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver configurado para que empates de nome não descartem entradas. (opcional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]