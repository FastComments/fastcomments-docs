[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός `Moderator` με βάση το `id`.

Η ενημέρωση ενός `Moderator` έχει τους ακόλουθους περιορισμούς:

- Οι ακόλουθες τιμές δεν μπορούν να παρέχονται κατά την ενημέρωση ενός `Moderator`:
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
- Δεν μπορείτε να αλλάξετε το `tenantId` που σχετίζεται με έναν `Moderator`.

[inline-code-attrs-start title = 'Moderator PATCH cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Moderator PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
