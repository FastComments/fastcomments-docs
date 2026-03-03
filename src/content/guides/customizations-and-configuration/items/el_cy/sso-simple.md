[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Με το Simple SSO, μπορούμε να παρέχουμε στο widget σχολιασμού πληροφορίες για τον χρήστη, ώστε να μην χρειάζεται να εισάγει το όνομα χρήστη ή το email του για να σχολιάσει.

Μπορούμε να διαμορφώσουμε το Simple SSO ως εξής:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Ο χρήστης θα είναι συνδεδεμένος, και θα δημιουργηθεί ένας SSO χρήστης στο παρασκήνιο. Ο χρήστης θα έχει `createdFromSimpleSSO` ορισμένο σε `true` εάν ελήφθη από το API.

Notes: 

- Το email είναι το μοναδικό αναγνωριστικό για το Simple SSO.
- Η παροχή email με το Simple SSO δεν είναι υποχρεωτική, ωστόσο από προεπιλογή τα σχόλιά τους θα εμφανίζονται ως «Μη επαληθευμένο». <b>Εάν δεν δοθεί email, ο χρήστης δεν μπορεί να πιστοποιηθεί πλήρως.</b>
- **ΝΕΟ** Από τον Ιανουάριο 2022: Τα ονόματα χρήστη δεν χρειάζεται να είναι μοναδικά σε όλο το fastcomments.com
- Το Simple SSO μπορεί αυτόματα να δημιουργεί και να ενημερώνει SSO χρήστες, εάν δοθεί email, και ο χρήστης δεν είχε αρχικά δημιουργηθεί μέσω Secure SSO.
- Μπορείτε να καθορίσετε badges για τον χρήστη με την ιδιότητα `badgeConfig`. Ο πίνακας `badgeIds` περιέχει τα IDs των παγκόσμιων badges που θα συσχετιστούν με τον χρήστη. Ο πίνακας `pageBadgeIds` περιέχει IDs badges που είναι περιορισμένα στην τρέχουσα σελίδα (`urlId`) — αυτά τα badges εμφανίζονται μόνο στη σελίδα όπου τους ανατέθηκαν. Εάν `override` οριστεί σε `true`, θα αντικαταστήσει τα υπάρχοντα εμφανιζόμενα badges (οι παγκόσμιες και οι σελίδας υπερκαλύπτονται ανεξάρτητα); αν `false`, θα προστεθούν στα υπάρχοντα badges.