Καταγράψτε μια ψήφο σε μια δημοσκόπηση, ή μετακινήστε μια υπάρχουσα σε διαφορετική επιλογή. Κάθε ψηφοφόρος έχει το πολύ μία ψήφο ανά δημοσκόπηση, οπότε η εκ νέου κλήση για τον ίδιο ψηφοφόρο μετακινεί την ψήφο του αντί να προσθέσει νέα.

Αυτό τηρεί τις ρυθμίσεις δημοσκοπήσεων του ιστότοπου: αν η ψηφοφορία έχει οριστεί μόνο για συνδεδεμένους χρήστες, μια ψήφος μόνο με anonUserId απορρίπτεται, και οι ανώνυμες ψήφοι υπόκεινται σε όριο ρυθμού ανά IP ανά δημοσκόπηση.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createPollVoteBody | CreatePollVoteBody | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // το userId είναι προαιρετικό και παραλείπεται εδώ
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]