## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| namespace | string | path | Tak |  |
| component | string | path | Tak |  |
| locale | string | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemów prosimy zgłaszać je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (opcjonalne)
let useFullTranslationIds = true // Bool |  (opcjonalne)

PublicAPI.getTranslations(namespace: namespace, component: component, locale: locale, useFullTranslationIds: useFullTranslationIds) { (response, error) in
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