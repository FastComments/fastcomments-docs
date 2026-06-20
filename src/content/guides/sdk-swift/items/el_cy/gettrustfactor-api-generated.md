## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserTrustFactorResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε ζήτημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

ModerationAPI.getTrustFactor(userId: userId, sso: sso) { (response, error) in
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