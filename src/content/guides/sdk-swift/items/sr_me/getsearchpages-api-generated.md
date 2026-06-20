## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationPageSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'getSearchPages Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (опционо)
let sso = "sso_example" // String |  (опционо)

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