## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getModerationCommentText'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O seguinte exemplo de código ainda está em beta. Para quaisquer problemas, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getModerationCommentText(tenantId: tenantId, commentId: commentId, sso: sso) { (response, error) in
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