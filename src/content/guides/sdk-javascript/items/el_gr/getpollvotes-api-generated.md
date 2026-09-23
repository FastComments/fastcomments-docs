The individual votes behind one poll's tallies, oldest first.

Οι μεμονωμένες ψήφοι πίσω από τα σύνολα μιας δημοσκόπησης, από την παλαιότερη πρώτη.

A poll belongs to a comment, so votes are always read one poll at a time - commentId is required. That keeps every query on the indexes the collection already has.

Μια δημοσκόπηση ανήκει σε ένα σχόλιο, έτσι οι ψήφοι διαβάζονται πάντα μία δημοσκόπηση τη φορά - απαιτείται το commentId. Αυτό διατηρεί κάθε ερώτημα στα ευρετήρια που ήδη έχει η συλλογή.

Obeys the poll's privacy: an anonymous poll's votes cannot be read (poll-anonymous), here or by id.

Τηρεί το απόρρητο της δημοσκόπησης: οι ψήφοι μιας ανώνυμης δημοσκόπησης δεν μπορούν να διαβαστούν (poll-anonymous), εδώ ή με το id.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| voterId | string | Όχι |  |
| optionId | string | Όχι |  |
| skip | number | Όχι |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---