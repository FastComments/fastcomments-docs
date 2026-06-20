## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| value | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSiteSearchResponse.swift)

## Primer

[inline-code-attrs-start title = 'getSearchSites Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite ga preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (opciono)
let sso = "sso_example" // String |  (opciono)

ModerationAPI.getSearchSites(value: value, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]