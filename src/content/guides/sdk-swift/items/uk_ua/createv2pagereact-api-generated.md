## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| id | string | query | Так |  |
| title | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Приклад

[inline-code-attrs-start title = 'createV2PageReact Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще в бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let id = "id_example" // String | 
let title = "title_example" // String |  (необов’язковий)
let sso = "sso_example" // String |  (необов’язковий)

PublicAPI.createV2PageReact(tenantId: tenantId, urlId: urlId, id: id, options: PublicAPI.CreateV2PageReactOptions(title: title, sso: sso)) { (response, error) in
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