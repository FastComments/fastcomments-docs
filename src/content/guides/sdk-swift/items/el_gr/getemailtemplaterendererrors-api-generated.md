## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateRenderErrorsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getEmailTemplateRenderErrors'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε ζήτημα, αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let skip = 987 // Double |  (προαιρετικό)

DefaultAPI.getEmailTemplateRenderErrors(tenantId: tenantId, id: id, skip: skip) { (response, error) in
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