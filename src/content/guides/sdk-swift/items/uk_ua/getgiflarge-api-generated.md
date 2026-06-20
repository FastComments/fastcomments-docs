## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| largeInternalURLSanitized | string | query | Так |  |

## Відповідь

Повертає: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GifGetLargeResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getGifLarge Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще є бета-версією. У разі проблем будь ласка повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let largeInternalURLSanitized = "largeInternalURLSanitized_example" // String | 

PublicAPI.getGifLarge(tenantId: tenantId, largeInternalURLSanitized: largeInternalURLSanitized) { (response, error) in
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