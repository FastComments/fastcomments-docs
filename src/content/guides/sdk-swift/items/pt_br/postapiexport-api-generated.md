---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|------------|----------|
| text-search | string | query | Não |  |
| byIPFromComment | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sorts | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opcional)
let byIPFromComment = "byIPFromComment_example" // String |  (opcional)
let filters = "filters_example" // String |  (opcional)
let searchFilters = "searchFilters_example" // String |  (opcional)
let sorts = "sorts_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.postApiExport(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso) { (response, error) in
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