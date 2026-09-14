[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Questo endpoint crea un singolo `FeedPost`. Ogni post ha un autore, quindi `fromUserId` è obbligatorio e deve essere l'ID di un utente FastComments o SSO esistente nell'account.

[inline-code-attrs-start title = 'Esempio cURL per Creare FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta di Creazione FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Invia il post ai feed aperti in un browser in questo momento. Predefinito a false. **/
    isLive?: boolean
    /** Esegui il post attraverso il motore anti-spam prima di salvarlo. Predefinito a false. **/
    doSpamCheck?: boolean
    /** Salta il controllo di contenuto ripetuto che viene eseguito come parte di doSpamCheck. Predefinito a false. **/
    skipDupCheck?: boolean
    /** Fino a 256 caratteri. Inviato agli ascoltatori live così un client può ignorare la propria trasmissione. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obbligatorio. Un ID utente FastComments o SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Sanitizzato al salvataggio. **/
    contentHTML?: string
    /** Sovrascrive il nome visualizzato preso dall'utente. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta di Creazione FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Incluso in caso di errore. **/
    reason?: string
    feedPost?: FeedPost; // Restituiamo il post completo creato in caso di successo.
}
[inline-code-end]