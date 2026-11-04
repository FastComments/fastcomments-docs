Comentadores anteriores na página que NÃO estão atualmente online. Ordenados por displayName.
Use isto após esgotar /users/online para renderizar uma seção "Members".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName via $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe o nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate do cursor: passe o nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador da URL da página (limpo no servidor).
let afterName = "afterName_example" // String | Cursor: passe o nextAfterName da resposta anterior. (optional)
let afterUserId = "afterUserId_example" // String | Desempate do cursor: passe o nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]