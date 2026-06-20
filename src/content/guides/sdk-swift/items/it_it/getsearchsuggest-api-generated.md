## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSuggestResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getSearchSuggest'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Gli esempi di codice seguenti sono ancora in beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

ModerationAPI.getSearchSuggest(textSearch: textSearch, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]