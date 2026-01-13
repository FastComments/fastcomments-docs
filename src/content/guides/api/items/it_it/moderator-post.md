[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di aggiungere un singolo `Moderator`.

La creazione di un `Moderator` presenta le seguenti restrizioni:

- Devono sempre essere forniti `name` e `email`. `userId` è opzionale.
- I seguenti valori non possono essere forniti durante la creazione di un `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Quando è specificato un `userId`, l'utente deve esistere.
- Quando è specificato un `userId`, questo deve appartenere allo stesso `tenantId` specificato nei parametri di query.
- Due moderatori nello stesso tenant non possono essere aggiunti con la stessa `email`.

Possiamo creare un `Moderator` per un utente di cui conosciamo solo l'email:

[inline-code-attrs-start title = 'Esempio cURL: Creazione Moderator tramite Email'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Oppure possiamo creare un `Moderator` per un utente che appartiene al nostro tenant, per monitorare le sue statistiche di moderazione:

[inline-code-attrs-start title = 'Esempio cURL: Creazione Moderator tramite utente del tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della richiesta per la creazione di Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per la creazione di Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    moderator?: Moderator; // Restituiamo il Moderator completo creato in caso di successo.
}
[inline-code-end]

---