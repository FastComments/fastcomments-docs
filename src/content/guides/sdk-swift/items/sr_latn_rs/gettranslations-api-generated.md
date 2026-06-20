## Parametri

| Name | Type | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| namespace | string | path | Da |  |
| component | string | path | Da |  |
| locale | string | query | Ne |  |
| useFullTranslationIds | boolean | query | Ne |  |

## Odgovor

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'getTranslations Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje u beta fazi. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (neobavezno)
let useFullTranslationIds = true // Bool |  (neobavezno)

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