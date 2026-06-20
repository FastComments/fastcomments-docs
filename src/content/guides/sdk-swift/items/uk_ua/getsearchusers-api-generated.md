## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationUserSearchResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще в бета-версії. Якщо виникає проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getSearchUsers(value: value, sso: sso) { (response, error) in
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