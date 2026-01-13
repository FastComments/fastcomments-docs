## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| locale | string | query | No |  |

## Respuesta

Devuelve: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RenderEmailTemplate200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de renderEmailTemplate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, repórtelo en http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let renderEmailTemplateBody = RenderEmailTemplateBody(emailTemplateId: "emailTemplateId_example", ejs: "ejs_example", testData: "TODO", translationOverridesByLocale: "TODO") // RenderEmailTemplateBody | 
let locale = "locale_example" // String |  (opcional)

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