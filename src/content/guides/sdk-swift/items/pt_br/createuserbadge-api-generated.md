## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateUserBadge200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createUserBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createUserBadgeParams = CreateUserBadgeParams(userId: "userId_example", badgeId: "badgeId_example", displayedOnComments: false) // CreateUserBadgeParams | 

DefaultAPI.createUserBadge(tenantId: tenantId, createUserBadgeParams: createUserBadgeParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]