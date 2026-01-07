[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δημιουργία ενός μόνο χρήστη SSO.

Η προσπάθεια δημιουργίας δύο χρηστών με το ίδιο ID θα οδηγήσει σε σφάλμα.

[inline-code-attrs-start title = 'Δημιουργία SSOUser cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

Σε αυτό το παράδειγμα καθορίζουμε `groupIds` για έλεγχο πρόσβασης, αλλά αυτό είναι προαιρετικό.

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Σημείωση Ενσωμάτωσης

Τα δεδομένα που περνούν από το API μπορούν να αντικατασταθούν απλά περνώντας ένα διαφορετικό φορτίο SSO User HMAC. Για παράδειγμα, αν
ορίσετε ένα username μέσω του API, αλλά μετά περάσετε ένα διαφορετικό μέσω της ροής SSO κατά τη φόρτωση σελίδας, θα ενημερώσουμε αυτόματα
το username τους.

Δεν θα ενημερώσουμε παραμέτρους χρήστη σε αυτή τη ροή εκτός αν τις καθορίσετε ρητά ή τις ορίσετε σε null (όχι undefined).
