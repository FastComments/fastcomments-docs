---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, effettua una segnalazione tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // Stringa |  (opzionale)
let byIPFromComment = "byIPFromComment_example" // Stringa |  (opzionale)
let filters = "filters_example" // Stringa |  (opzionale)
let searchFilters = "searchFilters_example" // Stringa |  (opzionale)
let sorts = "sorts_example" // Stringa |  (opzionale)
let sso = "sso_example" // Stringa |  (opzionale)

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