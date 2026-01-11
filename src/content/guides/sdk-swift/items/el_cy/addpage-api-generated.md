## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddPageAPIResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addPage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIPageData = CreateAPIPageData(accessibleByGroupIds: ["accessibleByGroupIds_example"], rootCommentCount: 123, commentCount: 123, title: "title_example", url: "url_example", urlId: "urlId_example") // CreateAPIPageData | 

DefaultAPI.addPage(tenantId: tenantId, createAPIPageData: createAPIPageData) { (response, error) in
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