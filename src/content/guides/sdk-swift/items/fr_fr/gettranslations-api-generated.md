## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Oui |  |
| component | string | path | Oui |  |
| locale | string | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |

## Réponse

Renvoie : [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (optionnel)
let useFullTranslationIds = true // Bool |  (optionnel)

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