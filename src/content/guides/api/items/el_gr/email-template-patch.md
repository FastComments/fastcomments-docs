[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός προτύπου email καθορίζοντας μόνο το id και τα χαρακτηριστικά προς ενημέρωση.

Σημειώστε ότι όλες οι ίδιες επικυρώσεις για τη δημιουργία προτύπου ισχύουν επίσης, για παράδειγμα:

- Το πρότυπο πρέπει να αποδίδεται. Αυτό ελέγχεται με κάθε ενημέρωση.
- Δεν μπορείτε να έχετε διπλότυπα πρότυπα για το ίδιο domain (διαφορετικά το ένα θα αγνοηθεί σιωπηλά).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The updated email template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
