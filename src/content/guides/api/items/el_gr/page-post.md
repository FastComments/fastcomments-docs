[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα δημιουργίας σελίδων.

Μια συνήθης περίπτωση χρήσης είναι ο έλεγχος πρόσβασης.

Σημειώσεις:

- Αν έχετε σχολιάσει σε ένα νήμα σχολίων, ή έχετε καλέσει το API για να δημιουργήσετε ένα `Comment`, έχετε ήδη δημιουργήσει ένα αντικείμενο `Page`! Μπορείτε να δοκιμάσετε να το ανακτήσετε μέσω
  της διαδρομής `Page` `/by-url-id`, περνώντας το ίδιο `urlId` που περάσατε στο widget σχολίων.
- Η δομή `Page` περιέχει ορισμένες **υπολογισμένες** τιμές.
  Αυτή τη στιγμή, αυτές είναι `commentCount` και `rootCommentCount`.
  Συμπληρώνονται αυτόματα και δεν μπορούν να οριστούν από το API. Η προσπάθεια να το κάνετε θα προκαλέσει το API να επιστρέψει σφάλμα.

[inline-code-attrs-start title = 'Page POST cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Page POST'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
