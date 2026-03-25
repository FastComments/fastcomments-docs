## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| usernameStartsWith | string | query | Não |  |
| mentionGroupIds | array | query | Não |  |
| sso | string | query | Não |  |
| searchSection | string | query | Não |  |

## Resposta

Retorna: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código abaixo ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (opcional)
let mentionGroupIds = ["inner_example"] // [String] |  (opcional)
let sso = "sso_example" // String |  (opcional)
let searchSection = "searchSection_example" // String |  (opcional)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]