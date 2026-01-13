[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Με το Simple SSO, μπορούμε να παρέχουμε στο widget σχολιασμού πληροφορίες για τον χρήστη ώστε να μην χρειάζεται να εισάγει το όνομα χρήστη ή το email του για να σχολιάσει.

Μπορούμε να ρυθμίσουμε το Simple SSO ως εξής:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Ο χρήστης θα έχει συνδεθεί, και θα δημιουργηθεί ένας SSO χρήστης στο παρασκήνιο. Ο χρήστης θα έχει `createdFromSimpleSSO` ορισμένο σε `true` αν ανακτηθεί από το API.

Σημειώσεις: 

- Το email είναι ο μοναδικός αναγνωριστής για το Simple SSO.
- Δεν απαιτείται να παρέχεται email με το Simple SSO, ωστόσο από προεπιλογή τα σχόλιά τους θα εμφανίζονται ως «Μη επαληθευμένο». <b>Εάν δεν δοθεί email, ο χρήστης δεν μπορεί να πιστοποιηθεί πλήρως.</b>
- **ΝΕΟ** Από τον Ιανουάριο 2022: Τα ονόματα χρήστη δεν χρειάζεται να είναι μοναδικά σε όλο το fastcomments.com
- Το Simple SSO μπορεί αυτόματα να δημιουργεί και να ενημερώνει SSO χρήστες, εφόσον παρέχεται email και ο χρήστης δεν δημιουργήθηκε αρχικά από το Secure SSO.
- Μπορείτε να καθορίσετε badges για τον χρήστη με την ιδιότητα `badgeConfig`. Ο πίνακας `badgeIds` περιέχει τα IDs των badges που θα συσχετιστούν με τον χρήστη. Εάν το `override` είναι ορισμένο σε `true`, θα αντικαταστήσει όλα τα υπάρχοντα badges που εμφανίζονται στα σχόλια· εάν σε `false`, θα προσθέσει στα υπάρχοντα badges.