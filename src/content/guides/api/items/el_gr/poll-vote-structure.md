A `PollVote` είναι η απάντηση ενός ατόμου σε μια δημοσκόπηση. Οι μετρήσεις που εμφανίζονται στην ίδια τη δημοσκόπηση διατηρούνται συγχρονισμένες με αυτά,
οπότε χρειάζεστε αυτά μόνο όταν θέλετε να ξέρετε *ποιος* ψήφισε τι, αντί για τα σύνολα.

Ένας ψηφοφόρος έχει το πολύ μία ψήφο ανά δημοσκόπηση. Η επαναψήφιση μετακινεί την υπάρχουσα ψήφο του στην νέα επιλογή αντί να προσθέτει μια δεύτερη, και το `updatedAt` καταγράφει πότε συνέβη αυτό.

`voterId` είναι το `userId` όταν ο ψηφοφόρος ήταν συνδεδεμένος, και το `anonUserId` διαφορετικά.

[inline-code-attrs-start title = 'Δομή PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** Το userId όταν ο ψηφοφόρος ήταν συνδεδεμένος, διαφορετικά το anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Όταν ο ψηφοφόρος μετακίνησε τελευταία τη ψήφο του σε διαφορετική επιλογή. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

Η ρύθμιση `privacy` της δημοσκόπησης εφαρμόζεται σε αυτό το API με τον ίδιο τρόπο που εφαρμόζεται στο widget σχολίων:

- **Anonymous** (η προεπιλογή): κανείς δεν μπορεί να δει πώς ψήφισε οποιοσδήποτε, έτσι οι ψήφοι δεν μπορούν να διαβαστούν.
  `GET /api/v1/poll-votes` and `GET /api/v1/poll-votes/:id` respond with `poll-anonymous`. The poll's counts
  are still available from `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: το κλειδί API σας ανήκει στον διαχειριστή του ιστότοπού σας, έτσι μπορεί να διαβάσει τις ψήφους.
- **Everyone**: οι ψήφοι μπορούν να διαβαστούν.

Η ιδιωτικότητα της δημοσκόπησης μπορεί να περιοριστεί αλλά όχι να επεκταθεί μόλις υπάρχουν ψήφοι.