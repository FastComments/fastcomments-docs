[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di aggiungere un singolo `Vote` autorizzato. I voti possono essere `up` (+1) o `down` (-1).

[inline-code-attrs-start title = 'Esempio cURL - Creazione di un voto'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL - Creazione di un voto anonimo'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta - Creazione voto'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta - Creazione voto'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Incluso in caso di errore. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Creazione di voti anonimi

I voti anonimi possono essere creati impostando `anonUserId` nei parametri di query invece di `userId`.

Questo id non deve corrispondere a un oggetto utente da nessuna parte (da qui anonimo). È semplicemente un identificatore
per la sessione, così puoi recuperare nuovamente i voti nella stessa sessione, per verificare se a un commento è stato
dato un voto.

Se non hai qualcosa come "sessioni anonime" come fa FastComments, puoi semplicemente
impostarlo su un ID casuale, come un UUID (anche se apprezziamo identificatori più piccoli per risparmiare spazio).

### Altre note

- Questa API rispetta le impostazioni a livello di tenant. Per esempio, se disabiliti il voto per una determinata pagina e tenti di creare un voto tramite l'API, fallirà con il codice di errore `voting-disabled`.
- Questa API è attiva di default.
- Questa API aggiornerà i `votes` del corrispondente `Comment`.