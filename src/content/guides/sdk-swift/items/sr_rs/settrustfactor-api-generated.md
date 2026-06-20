## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| userId | string | query | Не |  |
| trustFactor | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су и даље у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (опционо)
let trustFactor = "trustFactor_example" // String |  (опционо)
let sso = "sso_example" // String |  (опционо)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
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