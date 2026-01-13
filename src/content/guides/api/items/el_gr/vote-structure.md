Ένα αντικείμενο `Vote` αντιπροσωπεύει μια ψήφο που αφέθηκε από έναν χρήστη.

Η σχέση μεταξύ σχολίων και ψήφων ορίζεται μέσω του `commentId`.

Η δομή για το αντικείμενο Vote είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]
