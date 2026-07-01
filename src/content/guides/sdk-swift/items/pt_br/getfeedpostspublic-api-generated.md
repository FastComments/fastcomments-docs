req
tenantId
afterId

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| afterId | string | query | Não |  |
| limit | integer | query | Não |  |
| tags | array | query | Não |  |
| sso | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeUserInfo | boolean | query | Não |  |

## Resposta

Retorna: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// As amostras de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let afterId = "afterId_example" // String |  (opcional)
let limit = 987 // Int |  (opcional)
let tags = ["inner_example"] // [String] |  (opcional)
let sso = "sso_example" // String |  (opcional)
let isCrawler = true // Bool |  (opcional)
let includeUserInfo = true // Bool |  (opcional)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, options: PublicAPI.GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]