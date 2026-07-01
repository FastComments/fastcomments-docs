## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| editKey | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicAPIGetCommentTextResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para quaisquer problemas, relate via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let editKey = "editKey_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.getCommentText(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetCommentTextOptions(editKey: editKey, sso: sso)) { (response, error) in
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