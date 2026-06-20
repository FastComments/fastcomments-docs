## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersCountResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getCounts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. У випадку проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (необов'язковий)

ModerationAPI.getCounts(sso: sso) { (response, error) in
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