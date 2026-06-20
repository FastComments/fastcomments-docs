## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APICreateUserBadgeResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createUserBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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