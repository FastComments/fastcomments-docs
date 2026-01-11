## Parâmetros

| Name | Type | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |

## Resposta

Retorna: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UnBlockCommentPublic200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de unBlockUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let unBlockFromCommentParams = UnBlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // UnBlockFromCommentParams | 
let userId = "userId_example" // String |  (opcional)
let anonUserId = "anonUserId_example" // String |  (opcional)

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