[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di aggiornare un `Moderator` tramite `id`.

L'aggiornamento di un `Moderator` ha le seguenti restrizioni:

- I seguenti valori non possono essere forniti durante l'aggiornamento di un `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Quando viene specificato un `userId`, l'utente deve esistere.
- Quando viene specificato un `userId`, questo deve appartenere allo stesso `tenantId` specificato nei parametri di query.
- Due moderatori nello stesso tenant non possono essere aggiunti con la stessa `email`.
- Non è possibile modificare il `tenantId` associato a un `Moderator`.

[inline-code-attrs-start title = 'Esempio cURL PATCH per Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta PATCH per Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta PATCH per Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]

---