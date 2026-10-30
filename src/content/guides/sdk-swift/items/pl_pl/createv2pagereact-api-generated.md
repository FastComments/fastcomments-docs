## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| id | string | query | Tak |  |
| title | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Przykład

[inline-code-attrs-start title = 'createV2PageReact Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let id = "id_example" // String | 
let title = "title_example" // String |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)

PublicAPI.createV2PageReact(tenantId: tenantId, urlId: urlId, id: id, options: PublicAPI.CreateV2PageReactOptions(title: title, sso: sso)) { (response, error) in
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