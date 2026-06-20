## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Sim |  |
| component | string | path | Sim |  |
| locale | string | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |

## Resposta

Retorna: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTranslationsResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTranslations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte em http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let namespace = "namespace_example" // String | 
let component = "component_example" // String | 
let locale = "locale_example" // String |  (opcional)
let useFullTranslationIds = true // Bool |  (opcional)

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