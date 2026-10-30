Record a vote on a poll, or move an existing one to a different option. A voter has at most one vote per poll, so calling this again for the same voter moves their vote rather than adding one.

Αυτό τηρεί τις ρυθμίσεις δημοσκοπήσεων του ιστότοπου: εάν η ψήφισή είναι περιορισμένη μόνο σε συνδεδεμένους χρήστες, μια ψήφος με μόνο anonUserId απορρίπτεται, και οι ανώνυμες ψήφοι περιορίζονται ανά IP ανά δημοσκόπηση.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|------------|
| tenantId | string | query | Ναι |  |

## Response

Returns: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreatePollVoteResponse.swift)

## Example

[inline-code-attrs-start title = 'createPollVote Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createPollVoteBody = CreatePollVoteBody(commentId: "commentId_example", optionId: "optionId_example", userId: "userId_example", anonUserId: "anonUserId_example", ip: "ip_example") // CreatePollVoteBody | 

DefaultAPI.createPollVote(tenantId: tenantId, createPollVoteBody: createPollVoteBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]