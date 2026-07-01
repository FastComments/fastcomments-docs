---
Μεταφόρτωση και αλλαγή μεγέθους εικόνας

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Προεπιλογή μεγέθους: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) |
| urlId | string | query | No | Αναγνωριστικό σελίδας από το οποίο γίνεται η μεταφόρτωση, για διαμόρφωση |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Προεπιλογή μεγέθους: \"Default\" (1000x1000px) ή \"CrossPlatform\" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) (προαιρετικό)
let urlId = "urlId_example" // String | Αναγνωριστικό σελίδας από το οποίο γίνεται η μεταφόρτωση, για διαμόρφωση (προαιρετικό)

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
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