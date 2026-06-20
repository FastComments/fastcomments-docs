## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSiteSearchResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSites'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще перебувають у бета-версії. У разі проблем будь ласка повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getSearchSites(value: value, sso: sso) { (response, error) in
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