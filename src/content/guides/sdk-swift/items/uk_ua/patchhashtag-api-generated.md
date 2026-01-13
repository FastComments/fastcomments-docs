## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tag | string | path | Так |  |
| tenantId | string | query | Ні |  |

## Відповідь

Повертає: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchHashTag200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад patchHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (необов'язково)
let updateHashTagBody = UpdateHashTagBody(tenantId: "tenantId_example", url: "url_example", tag: "tag_example") // UpdateHashTagBody |  (необов'язково)

DefaultAPI.patchHashTag(tag: tag, tenantId: tenantId, updateHashTagBody: updateHashTagBody) { (response, error) in
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