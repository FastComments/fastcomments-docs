## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getSearchCommentsSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em versão beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (opcional)
let filters = "filters_example" // String |  (opcional)
let searchFilters = "searchFilters_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
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