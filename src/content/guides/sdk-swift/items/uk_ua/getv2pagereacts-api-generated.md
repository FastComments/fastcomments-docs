## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Коли встановлено, повертає користувачів для кожного react id (аватари та імена користувачів). Приймається лише \"public\", що виключає приватні профілі. |

## Відповідь

Повертає: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (необов'язково)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Коли встановлено, повертає користувачів для кожного react id (аватари та імена користувачів). Приймається лише \"public\", що виключає приватні профілі. (необов'язково)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]