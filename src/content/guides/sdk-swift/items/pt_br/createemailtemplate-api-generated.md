## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateEmailTemplate200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createEmailTemplate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em versão beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
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