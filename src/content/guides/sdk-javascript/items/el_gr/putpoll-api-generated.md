Επισυνάψτε μια δημοσκόπηση σε ένα σχόλιο, ή ορίστε την πλήρη κατάσταση της δημοσκόπησης που ήδη έχει.

Οι επιλογές αντιστοιχίζονται με βάση το id: μια επιλογή που αποστέλλεται με το id μιας υπάρχουσας επιλογής διατηρεί τις ψήφους της (και παίρνει τη νέα ετικέτα και θέση), μια επιλογή που αποστέλλεται χωρίς id προστίθεται, και οι υπάρχουσες επιλογές που παραλείπονται από τη λίστα αφαιρούνται μαζί με τις ψήφους που έχουν καταψηφιστεί σε αυτές.

Αν δεν διατηρηθεί κανένα υπάρχον id επιλογής σε μια δημοσκόπηση που έχει ψήφους, διαγράφονται όλες, οπότε αυτό απαιτεί replaceVotes=true.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| commentPollPutInput | CommentPollPutInput | Ναι |  |
| replaceVotes | boolean | Όχι |  |

## Απάντηση

Επιστρέφει: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]