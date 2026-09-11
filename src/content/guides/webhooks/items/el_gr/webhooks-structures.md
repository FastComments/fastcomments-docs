Η μόνη δομή που αποστέλλεται μέσω webhooks είναι το αντικείμενο WebhookComment, περιγραφόμενο σε TypeScript παρακάτω.

#### Η Δομή του Αντικειμένου WebhookComment

##### Η Δομή του Γεγονότος "Create"
Το σώμα του αιτήματος του γεγονότος "create" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Γεγονότος "Update"
Το σώμα του αιτήματος του γεγονότος "update" είναι ένα αντικείμενο WebhookComment.

##### Η Δομή του Γεγονότος "Delete"
Το σώμα του αιτήματος του γεγονότος "delete" είναι ένα αντικείμενο WebhookComment.

    Αλλαγή από 14 Νοεμβρίου 2023
    Πριν, το σώμα του αιτήματος του γεγονότος "delete" περιείχε μόνο το id του σχολίου. Τώρα περιέχει ολόκληρο το σχόλιο τη στιγμή της διαγραφής.

Κάθε κλειδί είναι πάντα παρόν στο σώμα. Όταν το σχόλιο δεν έχει τιμή για ένα πεδίο, το σώμα μεταφέρει `null` (ή `false` για boolean και `[]` για λίστες), έτσι η δομή μιας παράδοσης δεν διαφέρει ποτέ από ένα σχόλιο στο άλλο.

[inline-code-attrs-start title = 'Το Αντικείμενο WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
}
[inline-code-end]

Όταν χρήστες επισημαίνονται σε ένα σχόλιο, οι πληροφορίες αποθηκεύονται σε μια λίστα που ονομάζεται `mentions`. Κάθε αντικείμενο σε αυτή τη λίστα έχει την ακόλουθη δομή.

[inline-code-attrs-start title = 'Το Αντικείμενο Αναφορών Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

#### Μέθοδοι HTTP

Μπορείτε να ρυθμίσετε τη μέθοδο HTTP για κάθε τύπο γεγονότος webhook στον πίνακα διαχείρισης:

- **Create Event**: POST ή PUT (προεπιλογή: PUT)
- **Update Event**: POST ή PUT (προεπιλογή: PUT)
- **Delete Event**: DELETE, POST ή PUT (προεπιλογή: DELETE)

Δεδομένου ότι όλα τα αιτήματα περιέχουν ένα ID, οι λειτουργίες Δημιουργίας και Ενημέρωσης είναι ιδεομερείς από προεπιλογή (PUT). Η επανάληψη του ίδιου αιτήματος Δημιουργίας ή Ενημέρωσης δεν πρέπει να δημιουργεί διπλότυπα αντικείμενα από την πλευρά σας.

#### Κεφαλίδες Αιτήματος

Κάθε αίτημα webhook περιλαμβάνει τις ακόλουθες κεφαλίδες:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Το Μυστικό API σας |
| `X-FastComments-Timestamp` | Χρονική σήμανση Unix (δευτερόλεπτα) όταν υπογράφηκε το αίτημα |
| `X-FastComments-Signature` | Υπογραφή HMAC‑SHA256 (`sha256=<hex>`) |

Δείτε το [Ασφάλεια & Διακριτικά API](/guide-webhooks.html#webhooks-api-tokens) για πληροφορίες σχετικά με την επαλήθευση της υπογραφής HMAC.