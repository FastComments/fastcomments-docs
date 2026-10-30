## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| sso | string | query | Não |  |
| includeUserInfo | string | query | Não | Quando definido, retorna os usuários para cada id de reação (avatares e nomes de usuário). Apenas \"public\" é aceito, o que omite perfis privados. |

## Resposta

Retorna: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (opcional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Quando definido, retorna os usuários para cada id de reação (avatares e nomes de usuário). Apenas \"public\" é aceito, o que omite perfis privados. (opcional)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
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