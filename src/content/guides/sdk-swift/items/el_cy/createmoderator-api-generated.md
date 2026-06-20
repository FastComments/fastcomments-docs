## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Response

Επιστρέφει: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateModeratorResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα createModerator'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα παραδείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε στο http://github.com/OpenAPITools/openapi-generator/issues/new
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