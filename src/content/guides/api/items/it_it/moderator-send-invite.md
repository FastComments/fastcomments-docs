[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Questa route permette di invitare un singolo `Moderator`.

Le seguenti restrizioni si applicano per inviare un'email di invito a un `Moderator`:
- Il `Moderator` deve già esistere.
- Il `fromName` non può superare i `100 characters`.

**Note:**
- Se esiste già un utente con l'email fornita, gli verrà inviato un invito a moderare i commenti del tuo tenant.
- Se un utente con l'email fornita **non esiste**, il link d'invito lo guiderà nella creazione del proprio account.
- L'invito scade dopo `30 days`.

Possiamo creare un `Moderator` per un utente di cui conosciamo solo l'email:

[inline-code-attrs-start title = 'Esempio di richiesta cURL per invito Moderatore'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Questo invierà un'email come `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struttura richiesta invito Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** L'email inviata all'utente apparirà come inviata da questo nome. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta invito Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

---