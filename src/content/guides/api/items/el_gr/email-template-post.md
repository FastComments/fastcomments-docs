[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα δημιουργίας προτύπων email.

Σημειώσεις:

- Δεν μπορείτε να έχετε πολλαπλά πρότυπα με το ίδιο `emailTemplateId` με το ίδιο domain.
- Αλλά μπορείτε να έχετε ένα πρότυπο με χαρακτήρα μπαλαντέρ (`domain` = `*` και ένα πρότυπο ειδικό για domain για το ίδιο `emailTemplateId`).
- Ο καθορισμός `domain` είναι σχετικός μόνο αν έχετε διαφορετικά domains, ή θέλετε να χρησιμοποιήσετε συγκεκριμένα πρότυπα για δοκιμές (`domain` ορισμένο σε `localhost` κλπ).
- Αν καθορίσετε `domain` πρέπει να ταιριάζει με ένα `DomainConfig`. Σε σφάλμα παρέχεται μια λίστα με έγκυρα domains.
- Η σύνταξη προτύπου είναι EJS και αποδίδεται με χρονικό όριο 500ms. Το P99 για απόδοση είναι <5ms, οπότε αν φτάσετε τα 500ms κάτι δεν πάει καλά.
- **Το πρότυπό σας πρέπει να αποδίδεται με τα δεδομένα `testData` που δώσατε** για να αποθηκευτεί. Τα σφάλματα απόδοσης συγκεντρώνονται και αναφέρονται στον πίνακα ελέγχου (σύντομα διαθέσιμο μέσω API).

Τα ελάχιστα απαιτούμενα δεδομένα για την προσθήκη ενός προτύπου είναι τα ακόλουθα:

[inline-code-attrs-start title = 'Ελάχιστο EmailTemplate POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Μπορεί να θέλετε να έχετε πρότυπα ανά ιστότοπο, οπότε ορίζετε το `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος EmailTemplate POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης EmailTemplate POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
