Comentadores anteriores na página que NÃO estão atualmente online. Ordenado por displayName.
Use isto depois de esgotar /users/online para renderizar uma seção "Membros".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName} a partir de afterName para frente usando $gt, sem custo de $skip.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificador da URL da página (limpo no servidor).
let afterName = "afterName_example" // String | Cursor: passe nextAfterName da resposta anterior. (opcional)
let afterUserId = "afterUserId_example" // String | Desempate de cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas. (opcional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]