Ένα αντικείμενο `EmailTemplate` αντιπροσωπεύει τη διαμόρφωση για ένα προσαρμοσμένο πρότυπο email, για έναν ενοικιαστή.

Το σύστημα θα επιλέξει το πρότυπο email προς χρήση μέσω:

- Του αναγνωριστικού τύπου, το ονομάζουμε `emailTemplateId`. Αυτές είναι σταθερές.
- Το `domain`. Πρώτα θα προσπαθήσουμε να βρούμε ένα πρότυπο για το domain στο οποίο σχετίζεται το αντικείμενο (όπως ένα `Comment`), και αν δεν βρεθεί αντιστοιχία τότε θα προσπαθήσουμε να βρούμε ένα πρότυπο όπου το domain είναι null ή `*`.

Η δομή για το αντικείμενο `EmailTemplate` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Προτύπου Email'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### Σημειώσεις

- Μπορείτε να λάβετε τις έγκυρες τιμές `emailTemplateId` από το endpoint `/definitions`.
- Το endpoint `/definitions` περιλαμβάνει επίσης τις προεπιλεγμένες μεταφράσεις και δεδομένα δοκιμής.
- Τα πρότυπα θα αποτύχουν να αποθηκευτούν με μη έγκυρη δομή ή δεδομένα δοκιμής.
