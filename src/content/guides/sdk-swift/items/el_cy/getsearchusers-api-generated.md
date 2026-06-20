## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationUserSearchResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSearchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.getSearchUsers(value: value, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]