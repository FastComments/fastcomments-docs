## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateModerator200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createModerator'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código todavía están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createModeratorBody = CreateModeratorBody(name: "name_example", email: "email_example", userId: "userId_example", moderationGroupIds: ["moderationGroupIds_example"]) // CreateModeratorBody | 

DefaultAPI.createModerator(tenantId: tenantId, createModeratorBody: createModeratorBody) { (response, error) in
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