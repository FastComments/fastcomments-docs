The individual votes behind one poll's tallies, oldest first.

Μια δημοσκόπηση ανήκει σε ένα σχόλιο, έτσι οι ψήφοι διαβάζονται πάντα μία δημοσκόπηση τη φορά - απαιτείται το commentId. Αυτό διατηρεί κάθε ερώτημα στα ευρετήρια που ήδη έχει η συλλογή.

Τηρεί το απόρρητο της δημοσκόπησης: οι ψήφοι μιας ανώνυμης δημοσκόπησης δεν μπορούν να διαβαστούν (poll-anonymous), εδώ ή με το id.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | query | Ναι |  |
| voterId | string | query | Όχι |  |
| optionId | string | query | Όχι |  |
| skip | integer | query | Όχι |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVotesResponse.swift)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getPollVotes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let voterId = "voterId_example" // String |  (προαιρετικό)
let optionId = "optionId_example" // String |  (προαιρετικό)
let skip = 987 // Int |  (προαιρετικό)

DefaultAPI.getPollVotes(tenantId: tenantId, commentId: commentId, options: DefaultAPI.GetPollVotesOptions(voterId: voterId, optionId: optionId, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]