Quem votou em qual opção. Enquetes anônimas nunca respondem, enquetes apenas para administradores respondem somente para um administrador ou moderador.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|------------|-----------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| urlId | string | query | Não |  |
| page | integer | query | Não |  |
| search | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Returns: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPollVoters'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let page = 987 // Int |  (optional)
let search = "search_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]