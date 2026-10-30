## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Коли встановлено, повертає користувачів, які вподобали сторінку (аватари та імена користувачів). Приймається лише \"public\", що виключає приватні профілі. |

## Відповідь

Повертає: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Приклад

[inline-code-attrs-start title = 'getV1PageLikes Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
```
// Наступні зразки коду все ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Коли встановлено, повертає користувачів, які вподобали сторінку (аватари та імена користувачів). Приймається лише \"public\", що виключає приватні профілі. (необов’язково)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

---