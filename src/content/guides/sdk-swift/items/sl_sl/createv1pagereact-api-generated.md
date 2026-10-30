## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| title | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vrne: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Primer

[inline-code-attrs-start title = 'Primer createV1PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za morebitne težave prosimo, da jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let title = "title_example" // String |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

PublicAPI.createV1PageReact(tenantId: tenantId, urlId: urlId, options: PublicAPI.CreateV1PageReactOptions(title: title, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]