---
## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantManualBadgesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getManualBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. У разі проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (необов'язково)

ModerationAPI.getManualBadges(sso: sso) { (response, error) in
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