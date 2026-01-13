[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la possibilità di aggiornare una singola `Page`. I commenti corrispondenti verranno aggiornati.

[inline-code-attrs-start title = 'Esempio cURL di aggiornamento della pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di aggiornamento della pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di aggiornamento della pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Incluso in caso di fallimento. **/
    reason?: string
    user?: Page; // Restituiamo la pagina aggiornata completa in caso di successo.
}
[inline-code-end]

#### Nota

Alcuni parametri nell'oggetto Page vengono aggiornati automaticamente. Questi sono i conteggi e gli attributi del titolo. I conteggi non possono essere aggiornati tramite l'API poiché sono valori calcolati. Il `title` della pagina può essere impostato tramite l'API, ma verrebbe sovrascritto se il widget dei commenti viene utilizzato su una pagina con lo stesso `urlId` e un titolo di pagina diverso.