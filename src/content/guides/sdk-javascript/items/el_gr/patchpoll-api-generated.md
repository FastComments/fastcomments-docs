Επεξεργασία μιας ψηφοφορίας επιτόπου, διατηρώντας τα αποτελέσματά της: αλλάξτε την ερώτηση, μετονομάστε μια επιλογή, κλείστε ή ανοίξτε ξανά την ψηφοφορία,
ή αλλάξτε ποιοι μπορούν να δουν τους ψηφοφόρους. Οι επιλογές προσδιορίζονται με id - για να προσθέσετε, να αφαιρέσετε ή να αναδιατάξετε επιλογές, στείλτε PUT με την πλήρη λίστα.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| commentPollPatch | CommentPollPatch | Ναι |  |

## Απάντηση

Επιστρέφει: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]