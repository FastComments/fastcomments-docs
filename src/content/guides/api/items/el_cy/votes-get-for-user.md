[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Επιτρέπει την ανάκτηση ψήφων που αφέθηκαν από έναν χρήστη σε ένα δεδομένο `urlId`. Δέχεται ένα `userId` που μπορεί να είναι οποιοσδήποτε χρήστης FastComments.com ή `SSO User`.

Αυτό είναι χρήσιμο αν θέλετε να δείξετε αν ένας χρήστης έχει ψηφίσει σε ένα σχόλιο. Κατά την ανάκτηση σχολίων, απλά καλέστε αυτό το API ταυτόχρονα για τον χρήστη με το
ίδιο `urlId`.

Αν χρησιμοποιείτε ανώνυμη ψηφοφορία τότε θα θέλετε να περάσετε το `anonUserId` αντί αυτού.

[inline-code-attrs-start title = 'Ψήφοι Για Χρήστη cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Ψήφοι Για Ανώνυμο Χρήστη cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Σημειώστε ότι οι ανώνυμες ψήφοι θα εμφανιστούν στη λίστα `appliedAuthorizedVotes`. Θεωρούνται εξουσιοδοτημένες αφού δημιουργήθηκαν μέσω του API με κλειδί API.

[inline-code-attrs-start title = 'Δομή Αιτήματος Ψήφων Για Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Ψήφων Για Χρήστη'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
