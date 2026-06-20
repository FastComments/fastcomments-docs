## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationPageSearchResponse.swift)

## Przykład

[inline-code-attrs-start title = 'getSearchPages Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W przypadku problemu, zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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