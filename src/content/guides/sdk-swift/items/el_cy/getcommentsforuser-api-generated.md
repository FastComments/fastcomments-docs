## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| userId | string | query | Όχι |  |
| direction | string | query | Όχι |  |
| repliesToUserId | string | query | Όχι |  |
| page | number | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentsForUserResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (προαιρετικό)
let direction = SortDirections() // SortDirections |  (προαιρετικό)
let repliesToUserId = "repliesToUserId_example" // String |  (προαιρετικό)
let page = 987 // Double |  (προαιρετικό)
let includei10n = true // Bool |  (προαιρετικό)
let locale = "locale_example" // String |  (προαιρετικό)
let isCrawler = true // Bool |  (προαιρετικό)

PublicAPI.getCommentsForUser(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler) { (response, error) in
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