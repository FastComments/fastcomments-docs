## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Response

Повертає: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplates200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplates'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета‑версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (необов'язково)

DefaultAPI.getEmailTemplates(tenantId: tenantId, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]