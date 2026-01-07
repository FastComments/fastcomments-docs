[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα προσθήκης ενός μόνο `Moderator`.

Η δημιουργία ενός `Moderator` έχει τους ακόλουθους περιορισμούς:

- Ένα `name` και `email` πρέπει πάντα να παρέχονται. Το `userId` είναι προαιρετικό.
- Οι ακόλουθες τιμές δεν μπορούν να παρέχονται κατά τη δημιουργία ενός `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Όταν καθορίζεται ένα `userId`, αυτός ο χρήστης πρέπει να υπάρχει.
- Όταν καθορίζεται ένα `userId`, πρέπει να ανήκει στο ίδιο `tenantId` που καθορίζεται στις παραμέτρους ερωτήματος.
- Δύο συντονιστές στον ίδιο ενοικιαστή δεν μπορούν να προστεθούν με το ίδιο `email`.

Μπορούμε να δημιουργήσουμε έναν `Moderator` για έναν χρήστη για τον οποίο γνωρίζουμε μόνο το email:

[inline-code-attrs-start title = 'Δημιουργία Συντονιστή μέσω Email cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Ή μπορούμε να δημιουργήσουμε έναν `Moderator` για έναν χρήστη που ανήκει στον ενοικιαστή μας, για να παρακολουθούμε τα στατιστικά συντονισμού του:

[inline-code-attrs-start title = 'Δημιουργία Συντονιστή μέσω Χρήστη Ενοικιαστή cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
