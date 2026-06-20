---
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| postIds | array | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UserReactsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserReactsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще на стадії бета. Якщо виникнуть проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

PublicAPI.getUserReactsPublic(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
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