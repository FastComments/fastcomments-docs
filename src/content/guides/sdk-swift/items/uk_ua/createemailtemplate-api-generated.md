---
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Повертає: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateEmailTemplateResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад createEmailTemplate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду все ще знаходяться у бета-версії. Якщо виявите проблему, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createEmailTemplateBody = CreateEmailTemplateBody(emailTemplateId: "emailTemplateId_example", displayName: "displayName_example", ejs: "ejs_example", domain: "domain_example", translationOverridesByLocale: "TODO", testData: "TODO") // CreateEmailTemplateBody | 

DefaultAPI.createEmailTemplate(tenantId: tenantId, createEmailTemplateBody: createEmailTemplateBody) { (response, error) in
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