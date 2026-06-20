## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| value | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationPageSearchResponse.swift)

## Primer

[inline-code-attrs-start title = 'getSearchPages Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (opciono)
let sso = "sso_example" // String |  (opciono)

ModerationAPI.getSearchPages(value: value, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]