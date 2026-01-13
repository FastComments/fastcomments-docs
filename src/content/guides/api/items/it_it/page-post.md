[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Questo endpoint API permette di creare pagine.

Un caso d'uso comune è il controllo degli accessi.

Note:

- Se hai commentato in un thread di commenti, o hai chiamato l'API per creare un `Comment`, hai già creato un oggetto `Page`! Puoi provare a recuperarlo tramite la route `Page` `/by-url-id`, passando lo stesso `urlId` fornito al widget dei commenti.
- La struttura `Page` contiene alcuni valori **calcolati**. Attualmente, questi sono `commentCount` e `rootCommentCount`. Vengono popolati automaticamente e non possono essere impostati tramite l'API. Tentare di farlo farà sì che l'API restituisca un errore.

[inline-code-attrs-start title = 'Esempio cURL POST per Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Incluso in caso di errore. **/
    reason?: string
    /** La pagina creata. **/
    page?: Page
}
[inline-code-end]

---