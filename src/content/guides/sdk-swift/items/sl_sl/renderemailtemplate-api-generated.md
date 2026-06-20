## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| locale | string | query | Ne |  |

## Odgovor

Vrača: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RenderEmailTemplateResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer renderEmailTemplate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za vse težave jih prijavite prek http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let renderEmailTemplateBody = RenderEmailTemplateBody(emailTemplateId: "emailTemplateId_example", ejs: "ejs_example", testData: "TODO", translationOverridesByLocale: "TODO") // RenderEmailTemplateBody | 
let locale = "locale_example" // String |  (izbirno)

DefaultAPI.renderEmailTemplate(tenantId: tenantId, renderEmailTemplateBody: renderEmailTemplateBody, locale: locale) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]