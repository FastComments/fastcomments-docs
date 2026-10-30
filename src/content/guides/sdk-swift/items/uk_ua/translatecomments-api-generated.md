Translates the given comments into the widget locale. Only available when the matching widget
customization rule has Comment Translation set to Automatic or Manual.

## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## Відповідь

Повертає: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CommentTranslationsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад translateComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. У разі будь‑якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let commentTranslationsRequest = CommentTranslationsRequest(commentIds: ["commentIds_example"]) // CommentTranslationsRequest | 
let sso = "sso_example" // String |  (необов’язково)
let locale = "locale_example" // String |  (необов’язково)

PublicAPI.translateComments(tenantId: tenantId, urlId: urlId, commentTranslationsRequest: commentTranslationsRequest, options: PublicAPI.TranslateCommentsOptions(sso: sso, locale: locale)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]