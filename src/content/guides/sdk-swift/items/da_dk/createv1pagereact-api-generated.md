## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| title | string | query | No |  |
| sso | string | query | No |  |

## Svar

Returnerer: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Eksempel

[inline-code-attrs-start title = 'createV1PageReact Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let title = "title_example" // String | (valgfri)
let sso = "sso_example" // String | (valgfri)

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

---