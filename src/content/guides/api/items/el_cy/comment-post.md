[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα δημιουργίας σχολίων.

Συνήθεις περιπτώσεις χρήσης είναι προσαρμοσμένα UI, ενσωματώσεις ή εισαγωγές.

Σημειώσεις:

- Αυτό το API μπορεί να ενημερώσει το comment widget "live" αν επιθυμείτε (αυτό αυξάνει το `creditsCost` από `1` σε `2`).
- Αυτό το API θα δημιουργήσει αυτόματα αντικείμενα χρηστών στο σύστημά μας αν παρέχεται email.
- Η προσπάθεια αποθήκευσης δύο σχολίων με διαφορετικά emails, αλλά το ίδιο username, θα οδηγήσει σε σφάλμα για το δεύτερο σχόλιο.
- Αν καθορίζετε `parentId`, και ένα θυγατρικό σχόλιο έχει `notificationSentForParent` ως false, **θα στείλουμε ειδοποιήσεις για το γονικό σχόλιο**. Αυτό γίνεται κάθε ώρα (συγκεντρώνουμε τις ειδοποιήσεις μαζί για να μειώσουμε τον αριθμό των emails που αποστέλλονται).
- Αν θέλετε να στείλετε emails καλωσορίσματος κατά τη δημιουργία χρηστών, ή emails επαλήθευσης σχολίων, ορίστε `sendEmails` σε `true` στις παραμέτρους query.
- Τα σχόλια που δημιουργούνται μέσω αυτού του API θα εμφανίζονται στις σελίδες Analytics και Moderation της εφαρμογής διαχείρισης.
- Οι "κακές λέξεις" εξακολουθούν να καλύπτονται στα ονόματα σχολιαστών και στο κείμενο σχολίου αν η ρύθμιση είναι ενεργοποιημένη.
- Τα σχόλια που δημιουργούνται μέσω αυτού του API μπορούν ακόμα να ελεγχθούν για spam αν επιθυμείτε.
- Διαμόρφωση όπως μέγιστο μήκος σχολίου, αν διαμορφωθεί μέσω της σελίδας διαχείρισης Customization Rule, θα εφαρμοστεί εδώ.

Τα ελάχιστα δεδομένα που απαιτούνται για υποβολή που θα εμφανίζονται στο comment widget, είναι τα εξής:

[inline-code-attrs-start title = 'Ελάχιστη Δημιουργία Σχολίου POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Ένα πιο ρεαλιστικό αίτημα μπορεί να μοιάζει με:

[inline-code-attrs-start title = 'Δημιουργία Σχολίου POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας Σχολίου POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας Σχολίου POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
