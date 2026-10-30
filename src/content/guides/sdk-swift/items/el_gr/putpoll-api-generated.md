---
Συνημμένο μιας δημοσκόπησης σε ένα σχόλιο, ή ορίστε την πλήρη κατάσταση της δημοσκόπησης που ήδη έχει.

Οι επιλογές ταιριάζουν με βάση το id: μια επιλογή που αποστέλλεται με το id μιας υπάρχουσας επιλογής διατηρεί τις ψήφους της (και λαμβάνει τη νέα ετικέτα και θέση), μια επιλογή που αποστέλλεται χωρίς id προστίθεται, και οι υπάρχουσες επιλογές που παραλείπονται από τη λίστα αφαιρούνται μαζί με τις ψήφους που έχουν κατατεθεί σε αυτές.

Η διατήρηση μη υπαρχόντων ids επιλογών σε μια δημοσκόπηση που έχει ψήφους διαγράφει όλες τις επιλογές, επομένως απαιτείται replaceVotes=true.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| replaceVotes | boolean | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`SavePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SavePollResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'putPoll Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let commentPollPutInput = CommentPollPutInput(question: "question_example", options: [CommentPollOptionInput(id: "id_example", label: "label_example")], closesAt: "closesAt_example", privacy: 123, requireVoteToSeeResults: false) // CommentPollPutInput | 
let replaceVotes = true // Bool |  (προαιρετικό)

DefaultAPI.putPoll(tenantId: tenantId, commentId: commentId, commentPollPutInput: commentPollPutInput, replaceVotes: replaceVotes) { (response, error) in
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