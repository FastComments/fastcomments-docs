## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| direction | string | query | Não |  |
| repliesToUserId | string | query | Não |  |
| page | number | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| isCrawler | boolean | query | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (opcional)
let direction = SortDirections() // SortDirections |  (opcional)
let repliesToUserId = "repliesToUserId_example" // String |  (opcional)
let page = 987 // Double |  (opcional)
let includei10n = true // Bool |  (opcional)
let locale = "locale_example" // String |  (opcional)
let isCrawler = true // Bool |  (opcional)

PublicAPI.getCommentsForUser(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler) { (response, error) in
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