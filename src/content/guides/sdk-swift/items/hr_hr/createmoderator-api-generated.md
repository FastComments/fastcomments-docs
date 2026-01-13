## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateModerator200Response.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer createModerator'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
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